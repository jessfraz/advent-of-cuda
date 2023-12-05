//!  Day 3: Gear Ratios

use std::collections::BTreeMap;

use anyhow::Result;

/// You and the Elf eventually reach a gondola lift station; he says the gondola
/// lift will take you up to the water source, but this is as far as he can
/// bring you. You go inside.
///
/// It doesn't take long to find the gondolas, but there seems to be a problem:
/// they're not moving.
///
/// "Aaah!"
///
/// You turn around to see a slightly-greasy Elf with a wrench and a look of
/// surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working
/// right now; it'll still be a while before I can fix it." You offer to help.
///
/// The engineer explains that an engine part seems to be missing from the
/// engine, but nobody can figure out which one. If you can add up all the part
/// numbers in the engine schematic, it should be easy to work out which part
/// is missing.
///
/// The engine schematic (your puzzle input) consists of a visual representation
/// of the engine. There are lots of numbers and symbols you don't really
/// understand, but apparently any number adjacent to a symbol, even diagonally,
/// is a "part number" and should be included in your sum. (Periods (.) do not
/// count as a symbol.)
///
/// Here is an example engine schematic:
///
///     467..114..
///     ...*......
///     ..35..633.
///     ......#...
///     617*......
///     .....+.58.
///     ..592.....
///     ......755.
///     ...$.*....
///     .664.598..
///
/// In this schematic, two numbers are not part numbers because they are not
/// adjacent to a symbol: 114 (top right) and 58 (middle right). Every other
/// number is adjacent to a symbol and so is a part number; their sum is 4361.
///
/// Of course, the actual engine schematic is much larger. What is the sum of
/// all of the part numbers in the engine schematic?
pub fn solve_part_1(input: &str) -> Result<u32> {
    let mut numbers_positions: BTreeMap<(i32, i32), u32> = BTreeMap::new();
    let mut symbols_positions: BTreeMap<(i32, i32), char> = BTreeMap::new();

    for (y, line) in input.lines().enumerate() {
        let mut previous_number = None;
        for (x, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => {
                    // Get the full number.
                    // Try to get the number on the right.
                    if previous_number.is_none() {
                        let number = line[x..]
                            .chars()
                            .take_while(|c| c.is_digit(10))
                            .collect::<String>()
                            .parse::<u32>()?;
                        numbers_positions.insert((y as i32, x as i32), number);
                        previous_number = Some(number);
                    }
                }
                '.' => {
                    previous_number = None;
                }
                _ => {
                    symbols_positions.insert((y as i32, x as i32), c);
                    previous_number = None;
                }
            }
        }
    }

    let mut adjacent_nums: Vec<u32> = Vec::new();
    // Iterare over the numbers and check if they are adjacent to a symbol.
    for ((line, pos), number) in numbers_positions {
        // Check if the number is adjacent to a symbol.
        let mut possible_places = vec![
            (line - 1, pos),
            (line + 1, pos),
            (line - 1, pos - 1),
            (line - 1, pos + 1),
            (line + 1, pos - 1),
            (line + 1, pos + 1),
            (line, pos - 1),
            (line, pos + 1),
        ];
        let num_str = number.to_string();
        for num in 1..num_str.len() {
            possible_places.push((line - 1, pos + num as i32));
            possible_places.push((line - 1, pos + num as i32 + 1));
            possible_places.push((line + 1, pos + num as i32));
            possible_places.push((line + 1, pos + num as i32 + 1));
            possible_places.push((line, pos + num as i32 + 1));
        }
        let mut adjacent = false;
        for (y, x) in possible_places {
            if symbols_positions.contains_key(&(y, x)) {
                adjacent = true;
                break;
            }
        }

        if adjacent {
            adjacent_nums.push(number);
        }
    }

    Ok(adjacent_nums.iter().sum())
}

/// The engineer finds the missing part and installs it in the engine! As the
/// engine springs to life, you jump in the closest gondola, finally ready to
/// ascend to the water source.
///
/// You don't seem to be going very fast, though. Maybe something is still wrong?
/// Fortunately, the gondola has a phone labeled "help", so you pick it up and
/// the engineer answers.
///
/// Before you can explain the situation, she suggests that you look out the
/// window. There stands the engineer, holding a phone in one hand and waving
/// with the other. You're going so slowly that you haven't even left the
/// station. You exit the gondola.
///
/// The missing part wasn't the only issue - one of the gears in the engine is
/// wrong. A gear is any * symbol that is adjacent to exactly two part numbers.
/// Its gear ratio is the result of multiplying those two numbers together.
///
/// This time, you need to find the gear ratio of every gear and add them all
/// up so that the engineer can figure out which gear needs to be replaced.
///
///Consider the same engine schematic again:
///
///     467..114..
///     ...*......
///     ..35..633.
///     ......#...
///     617*......
///     .....+.58.
///     ..592.....
///     ......755.
///     ...$.*....
///     .664.598..
///
/// In this schematic, there are two gears. The first is in the top left;
/// it has part numbers 467 and 35, so its gear ratio is 16345. The second gear
/// is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is
/// not a gear because it is only adjacent to one part number.) Adding up all of
/// the gear ratios produces 467835.
///
/// What is the sum of all of the gear ratios in your engine schematic?
pub fn solve_part_2(input: &str) -> Result<u32> {
    let mut numbers_positions: BTreeMap<(i32, i32), u32> = BTreeMap::new();
    let mut symbols_positions: BTreeMap<(i32, i32), char> = BTreeMap::new();

    for (y, line) in input.lines().enumerate() {
        let mut previous_number = None;
        for (x, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => {
                    // Get the full number.
                    // Try to get the number on the right.
                    if let Some(previous) = previous_number {
                        numbers_positions
                            .insert((y as i32, x as i32), previous);
                    } else {
                        let number = line[x..]
                            .chars()
                            .take_while(|c| c.is_digit(10))
                            .collect::<String>()
                            .parse::<u32>()?;
                        numbers_positions.insert((y as i32, x as i32), number);
                        previous_number = Some(number);
                    }
                }
                '*' => {
                    symbols_positions.insert((y as i32, x as i32), c);
                    previous_number = None;
                }
                _ => {
                    previous_number = None;
                }
            }
        }
    }

    let mut gear_ratios: Vec<u32> = Vec::new();
    for ((line, pos), _) in symbols_positions {
        // Check if two different numbers are adjacent to the symbol.
        let possible_places = vec![
            (line - 1, pos),
            (line + 1, pos),
            (line - 1, pos - 1),
            (line - 1, pos + 1),
            (line + 1, pos - 1),
            (line + 1, pos + 1),
            (line, pos - 1),
            (line, pos + 1),
        ];
        let mut adjacent_numbers: Vec<u32> = Vec::new();
        for (y, x) in possible_places {
            if let Some(number) = numbers_positions.get(&(y, x)) {
                if !adjacent_numbers.contains(number) {
                    adjacent_numbers.push(*number);
                }
            }
        }

        if adjacent_numbers.len() == 2 {
            gear_ratios.push(adjacent_numbers.iter().product());
        }
    }

    Ok(gear_ratios.iter().sum())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        // Load the file.
        let input = include_str!("../input/day03.txt");
        assert_eq!(super::solve_part_1(input).unwrap(), 535351);
    }

    #[test]
    fn test_solve_part_2() {
        // Load the file.
        let input = include_str!("../input/day03.txt");
        assert_eq!(super::solve_part_2(input).unwrap(), 87287096);
    }
}
