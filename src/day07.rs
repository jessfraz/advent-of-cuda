//!  Day 7:

use anyhow::Result;

/// Some docs here.
pub fn solve_part_1(_input: &str) -> Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        // Load the file.
        let input = include_str!("../input/day07.txt");
        assert_eq!(super::solve_part_1(input).unwrap(), 21485);
    }
}
