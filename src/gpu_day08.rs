//!  Solve for day 08: Haunted Wasteland using a GPU.
use std::collections::BTreeMap;

use anyhow::Result;
use cust::prelude::*;

/// Do the graph traversal on the GPU.
fn graph_traversal(
    graph: &[(u32, u32)],
    start: &[u32],
    goals: &[u32],
    directions: &[bool], // true for left, false for right
) -> Result<(Vec<u32>, Vec<u64>)> {
    // Initialize CUDA, this will pick the first available device and will
    // make a CUDA context from it.
    // We don't need the context for anything but it must be kept alive.
    let _ctx = cust::quick_init()?;

    // Make the CUDA module, modules just house the GPU code for the kernels we
    // created. they can be made from PTX code, cubins, or fatbins.
    let module = Module::from_ptx(crate::PTX, &[])?;

    // make a CUDA stream to issue calls to. You can think of this as an OS
    // thread but for dispatching GPU calls.
    let stream = Stream::new(StreamFlags::NON_BLOCKING, None)?;

    // allocate the GPU memory needed to house our numbers and copy them over.
    let graph_gpu = graph.as_dbuf()?;
    let start_gpu = start.as_dbuf()?;
    let goals_gpu = goals.as_dbuf()?;
    let directions_gpu = directions.as_dbuf()?;

    let length = start.len();

    // Allocate our output buffer. You could also use
    // DeviceBuffer::uninitialized() to avoid the cost of the copy, but you
    // need to be careful not to read from the buffer.
    let mut nodes = vec![0u32; length];
    let nodes_buf = nodes.as_slice().as_dbuf()?;

    let mut steps = vec![0u64; length];
    let steps_buf = steps.as_slice().as_dbuf()?;

    // Retrieve the kernel from the module so we can calculate the right
    // launch config.
    let func = module.get_function("graph_traversal")?;

    // Use the CUDA occupancy API to find an optimal launch configuration for
    // the grid and block size. This will try to maximize how much of the
    // GPU is used by finding the best launch configuration for the
    // current CUDA device/architecture.
    let (_, block_size) = func.suggested_launch_configuration(0, 0.into())?;

    let grid_size = (length as u32 + block_size - 1) / block_size;

    println!(
        "using {} blocks and {} threads per block",
        grid_size, block_size
    );

    // Actually launch the GPU kernel. This will queue up the launch on the
    // stream, it will not block the thread until the kernel is finished.
    unsafe {
        launch!(
            // slices are passed as two parameters, the pointer and the length.
            func<<<grid_size, block_size, 0, stream>>>(
                graph_gpu.as_device_ptr(),
                graph_gpu.len(),
                start_gpu.as_device_ptr(),
                start_gpu.len(),
                goals_gpu.as_device_ptr(),
                goals_gpu.len(),
                directions_gpu.as_device_ptr(),
                directions_gpu.len(),
                nodes_buf.as_device_ptr(),
                steps_buf.as_device_ptr(),
            )
        )?;
    }

    stream.synchronize()?;

    // copy back the data from the GPU.
    nodes_buf.copy_to(&mut nodes)?;
    steps_buf.copy_to(&mut steps)?;

    Ok((nodes, steps))
}

/// The sandstorm is upon you and you aren't any closer to escaping the
/// wasteland. You had the camel follow the instructions, but you've barely left
/// your starting position. It's going to take *significantly more steps* to
/// escape!
///
/// What if the map isn't for people - what if the map is for *ghosts*? Are
/// ghosts even bound by the laws of spacetime? Only one way to find out.
///
/// After examining the maps a bit longer, your attention is drawn to a curious
/// fact: the number of nodes with names ending in `A` is equal to the number
/// ending in `Z`! If you were a ghost, you'd probably just *start at every node
/// that ends with `A`* and follow all of the paths at the same time until they
/// all simultaneously end up at nodes that end with `Z`.
///
/// For example:
///
/// ```ignore
/// LR
///
/// 11A = (11B, XXX)
/// 11B = (XXX, 11Z)
/// 11Z = (11B, XXX)
/// 22A = (22B, XXX)
/// 22B = (22C, 22C)
/// 22C = (22Z, 22Z)
/// 22Z = (22B, 22B)
/// XXX = (XXX, XXX)
/// ```
///
/// Here, there are two starting nodes, `11A` and `22A` (because they both end
/// with `A`). As you follow each left/right instruction, use that instruction
/// to *simultaneously* navigate away from both nodes you're currently on.
/// Repeat this process until *all* of the nodes you're currently on end with
/// `Z`. (If only some of the nodes you're on end with `Z`, they act like any
/// other node and you continue as normal.) In this example, you would proceed
/// as follows:
///
/// * Step 0: You are at `11A` and `22A`.
/// * Step 1: You choose all of the *left* paths, leading you to `11B` and
///   `22B`.
/// * Step 2: You choose all of the *right* paths, leading you to `*11Z*` and
///   `22C`.
/// * Step 3: You choose all of the *left* paths, leading you to `11B` and
///   `*22Z*`.
/// * Step 4: You choose all of the *right* paths, leading you to `*11Z*` and
///   `22B`.
/// * Step 5: You choose all of the *left* paths, leading you to `11B` and
///   `22C`.
/// * Step 6: You choose all of the *right* paths, leading you to `*11Z*` and
///   `*22Z*`.
///
/// So, in this example, you end up entirely on nodes that end in `Z` after
/// `*6*` steps.
///
/// Simultaneously start on every node that ends with `A`. *How many steps does
/// it take before you're only on nodes that end with `Z`?*
pub fn solve_part_2(input: &str) -> Result<u64> {
    let (directions, instructions) =
        crate::day08::parse_directions_and_instructions(input)?;

    // Create the graph.
    let instruction_keys = instructions
        .keys()
        .iter()
        .enumerate()
        .map(|(index, key)| (**key, index as u32))
        .collect::<Vec<([char; 3], u32)>>();
    let instruction_keys_map = BTreeMap::from_iter(instruction_keys);

    let graph: Vec<(u32, u32)> = instructions
        .values()
        .iter()
        .map(|instruction| {
            (
                *instruction_keys_map.get(&instruction.0).unwrap(),
                *instruction_keys_map.get(&instruction.1).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    // Find all the ending Z node indexes.
    let goals: Vec<u32> = instructions
        .keys()
        .iter()
        .enumerate()
        .filter_map(|(index, label)| {
            if label[2] == 'Z' {
                Some(index as u32)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Change the direction into an array of bools.
    let directions: Vec<bool> = directions
        .iter()
        .map(|direction| match direction {
            crate::day08::Direction::Left => true,
            crate::day08::Direction::Right => false,
        })
        .collect::<Vec<_>>();

    // Find all the starting A node indexes.
    let mut start: Vec<u32> = instructions
        .keys()
        .iter()
        .enumerate()
        .filter_map(|(index, label)| {
            if label[2] == 'A' {
                Some(index as u32)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut steps = 0;
    loop {
        let (nodes, steps_run) =
            graph_traversal(&graph, &start, &goals, &directions)?;

        steps += steps_run.len() as u64;

        if nodes.iter().all(|node| goals.contains(node)) {
            break;
        }

        start = nodes;
    }

    Ok(steps)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_2() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
        assert_eq!(super::solve_part_2(input).unwrap(), 6);

        // Load the file.
        //let input = include_str!("../input/day08.txt");
        //assert_eq!(super::solve_part_2(input).unwrap(), 15726453850399);
    }
}
