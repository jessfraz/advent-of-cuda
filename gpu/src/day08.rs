//!  CUDA Kernel for day 08: Haunted Wasteland

use cuda_std::*;

/// Traverse the graph from nodes ending with A until all nodes reach nodes
/// ending with Z.
#[kernel]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn graph_traversal(
    graph: &[(u32, u32)],
    start: &[u32],
    goals: &[u32],
    directions: &[bool], // true for left, false for right
    nodes: *mut u32,
    steps: *mut u64,
) {
    let nodes_len = goals.len();
    let mut items = start.to_vec();
    for i in directions {
        let idx = thread::index_1d() as usize;
        let mut found = true;

        if idx < nodes_len {
            let mut node = items[idx];
            if *i {
                node = graph[node as usize].0;
            } else {
                node = graph[node as usize].1;
            }

            if found && !goals.contains(&node) {
                found = false;
            }

            items.insert(idx, node);
            let elem = &mut *nodes.add(idx);
            *elem = node;
            let elem = &mut *steps.add(idx);
            *elem = 1;
        }

        if found && thread::first() {
            break;
        }
    }
}
