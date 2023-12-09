//!  Day 08: Haunted Wasteland
use std::collections::BTreeMap;

use anyhow::Result;
use rayon::prelude::*;

/// The direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Left.
    Left,
    /// Right.
    Right,
}

impl Direction {
    /// Parse a direction from a character.
    pub fn parse(c: char) -> Result<Self> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => anyhow::bail!("invalid direction: {}", c),
        }
    }
}

/// A map of labels to their instructions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionMap(BTreeMap<[char; 3], ([char; 3], [char; 3])>);

impl InstructionMap {
    /// Get the next instruction.
    pub fn get(
        &self,
        instruction: &[char; 3],
    ) -> Result<&([char; 3], [char; 3])> {
        self.0.get(instruction).ok_or_else(|| {
            anyhow::anyhow!("invalid instruction: {:?}", instruction)
        })
    }

    /// Get the next left instruction.
    pub fn get_left(&self, instruction: &[char; 3]) -> Result<[char; 3]> {
        Ok(self.get(instruction)?.0)
    }

    /// Get the next right instruction.
    pub fn get_right(&self, instruction: &[char; 3]) -> Result<[char; 3]> {
        Ok(self.get(instruction)?.1)
    }

    /// Get the values for the instructions.
    pub fn values(&self) -> Vec<&([char; 3], [char; 3])> {
        self.0.values().collect::<Vec<_>>()
    }

    /// Get the keys for the instructions.
    pub fn keys(&self) -> Vec<&[char; 3]> {
        self.0.keys().collect::<Vec<_>>()
    }
}

/// An instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    /// The label.
    pub label: [char; 3],
    /// The values.
    pub values: ([char; 3], [char; 3]),
}

impl Instruction {
    /// Parse an instruction from a string.
    pub fn parse(s: &str) -> Result<Self> {
        let mut parts = s.split(" = ");
        let label = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing label"))?
            .trim()
            .to_string();
        let values = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing values"))?
            .trim_matches(|c| c == '(' || c == ')');

        let mut values = values.split(", ");
        let values = (
            values
                .next()
                .ok_or_else(|| anyhow::anyhow!("missing first value"))?
                .chars()
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| anyhow::anyhow!("invalid first value"))?,
            values
                .next()
                .ok_or_else(|| anyhow::anyhow!("missing second value"))?
                .chars()
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| anyhow::anyhow!("invalid second value"))?,
        );

        // Parse the values.
        Ok(Instruction {
            label: label
                .chars()
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| anyhow::anyhow!("invalid label"))?,
            values,
        })
    }
}

/// Parse the directions and instructions.
pub fn parse_directions_and_instructions(
    input: &str,
) -> Result<(Vec<Direction>, InstructionMap)> {
    let mut lines = input.lines();

    // Parse the direction.
    let directions = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("missing direction"))?
        .chars()
        .map(Direction::parse)
        .collect::<Result<Vec<_>>>()?;

    // Parse the instructions.
    let mut instructions = BTreeMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let instruction = Instruction::parse(line)?;
        instructions.insert(instruction.label, instruction.values);
    }

    Ok((directions, InstructionMap(instructions)))
}

// Function to find the greatest common divisor (GCD) using Euclidean algorithm.
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a // If b is 0, a is the GCD.
    } else {
        gcd(b, a % b) // Recursive call with b and remainder of a divided by b.
    }
}

// Function to find the least common multiple (LCM) using GCD.
fn lcm(a: u64, b: u64) -> u64 {
    // LCM is calculated using the formula: (a * b) / GCD(a, b).
    (a * b) / gcd(a, b)
}

// Function to calculate LCM of a vector of cycle lengths.
fn calculate_lcm(cycle_lengths: Vec<u64>) -> u64 {
    cycle_lengths.iter().fold(1, |a, b| lcm(a, *b))
}

/// You're still riding a camel across Desert Island when you spot a sandstorm
/// quickly approaching. When you turn to warn the Elf, she disappears before
/// your eyes! To be fair, she had just finished warning you about *ghosts* a
/// few minutes ago.
///
/// One of the camel's pouches is labeled "maps" - sure enough, it's full of
/// documents (your puzzle input) about how to navigate the desert. At least,
/// you're pretty sure that's what they are; one of the documents contains a
/// list of left/right instructions, and the rest of the documents seem to
/// describe some kind of *network* of labeled nodes.
///
/// It seems like you're meant to use the *left/right* instructions to *navigate
/// the network*. Perhaps if you have the camel follow the same instructions,
/// you can escape the haunted wasteland!
///
/// After examining the maps for a bit, two nodes stick out: `AAA` and `ZZZ`.
/// You feel like `AAA` is where you are now, and you have to follow the
/// left/right instructions until you reach `ZZZ`.
///
/// This format defines each *node* of the network individually. For example:
///
/// ```ignore
/// RL
///
/// AAA = (BBB, CCC)
/// BBB = (DDD, EEE)
/// CCC = (ZZZ, GGG)
/// DDD = (DDD, DDD)
/// EEE = (EEE, EEE)
/// GGG = (GGG, GGG)
/// ZZZ = (ZZZ, ZZZ)
/// ```
///
/// Starting with `AAA`, you need to *look up the next element* based on the
/// next left/right instruction in your input. In this example, start with `AAA`
/// and go *right* (`R`) by choosing the right element of `AAA`, `*CCC*`. Then,
/// `L` means to choose the *left* element of `CCC`, `*ZZZ*`. By following the
/// left/right instructions, you reach `ZZZ` in `*2*` steps.
///
/// Of course, you might not find `ZZZ` right away. If you run out of left/right
/// instructions, repeat the whole sequence of instructions as necessary: `RL`
/// really means `RLRLRLRLRLRLRLRL...` and so on. For example, here is a
/// situation that takes `*6*` steps to reach `ZZZ`:
///
/// ```ignore
/// LLR
///
/// AAA = (BBB, BBB)
/// BBB = (AAA, ZZZ)
/// ZZZ = (ZZZ, ZZZ)
/// ```
///
/// Starting at `AAA`, follow the left/right instructions. *How many steps are
/// required to reach `ZZZ`?*
pub fn solve_part_1(input: &str) -> Result<u64> {
    let (directions, instructions) = parse_directions_and_instructions(input)?;

    // Now solve the puzzle.
    let mut current_instruction = ['A', 'A', 'A'];
    let mut steps = 0;
    while current_instruction != ['Z', 'Z', 'Z'] {
        // Get the direction.
        let direction = if steps >= directions.len() {
            // Get the next instruction modulo the number of instructions.
            directions
                .get(steps % directions.len())
                .ok_or_else(|| anyhow::anyhow!("invalid direction"))?
        } else {
            directions
                .get(steps)
                .ok_or_else(|| anyhow::anyhow!("invalid direction"))?
        };

        steps += 1;

        // Get the next instruction.
        current_instruction = match direction {
            Direction::Left => instructions.get_left(&current_instruction)?,
            Direction::Right => instructions.get_right(&current_instruction)?,
        };
    }

    Ok(steps as u64)
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
    let (directions, instructions) = parse_directions_and_instructions(input)?;

    // Find all the instructions that end in `A`.
    let mut current_instructions = instructions
        .0
        .par_iter()
        .filter(|(k, (_, _))| k[2] == 'A')
        .map(|(k, _)| *k)
        .collect::<Vec<_>>();
    let mut steps = 0;
    let mut cycle_lengths: BTreeMap<[char; 3], u64> = BTreeMap::new();
    while cycle_lengths.len() != current_instructions.len() {
        // Get the direction.
        let direction = if steps >= directions.len() {
            // Get the next instruction modulo the number of instructions.
            directions
                .get(steps % directions.len())
                .ok_or_else(|| anyhow::anyhow!("invalid direction"))?
        } else {
            directions
                .get(steps)
                .ok_or_else(|| anyhow::anyhow!("invalid direction"))?
        };

        steps += 1;

        for instruction in current_instructions.iter_mut() {
            // Get the next instruction.
            *instruction = match direction {
                Direction::Left => instructions.get_left(instruction).unwrap(),
                Direction::Right => {
                    instructions.get_right(instruction).unwrap()
                }
            };
            if instruction[2] == 'Z' {
                // We've reached the end of a cycle for this node.
                let cycle_length = steps;
                cycle_lengths.insert(*instruction, cycle_length as u64);
            }
        }
    }

    Ok(calculate_lcm(cycle_lengths.values().copied().collect()))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        // Load the file.
        let input = include_str!("../input/day08.txt");
        assert_eq!(super::solve_part_1(input).unwrap(), 16043);
    }

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
        let input = include_str!("../input/day08.txt");
        assert_eq!(super::solve_part_2(input).unwrap(), 15726453850399);
    }
}
