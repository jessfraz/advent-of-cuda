//!  Day 01: Trebuchet?!
use anyhow::Result;

/// Return the first number in a sequence of chars.
fn first_number(chars: &std::str::Chars) -> Result<u32> {
    let mut chars2 = chars.clone();
    for c in chars2.by_ref() {
        if c.is_ascii_digit() {
            let number = Some(
                c.to_digit(10)
                    .ok_or_else(|| anyhow::anyhow!("Invalid digit: {}", c))?,
            );
            return number
                .ok_or_else(|| anyhow::anyhow!("No number found: {}", c));
        }
    }

    anyhow::bail!("No first number found: {}", chars.as_str())
}

/// Return the last number in a sequence of chars.
fn last_number(chars: &std::str::Chars) -> Result<u32> {
    let mut chars2 = chars.clone();
    while let Some(c) = chars2.next_back() {
        if c.is_ascii_digit() {
            let number = Some(
                c.to_digit(10)
                    .ok_or_else(|| anyhow::anyhow!("Invalid digit: {}", c))?,
            );
            return number
                .ok_or_else(|| anyhow::anyhow!("No number found: {}", c));
        }
    }

    anyhow::bail!("No last number found: {}", chars.as_str())
}

/// Something is wrong with global snow production, and you've been selected to
/// take a look. The Elves have even given you a map; on it, they've used stars
/// to mark the top fifty locations that are likely to be having problems.
///
/// You've been doing this long enough to know that to restore snow operations,
/// you need to check all *fifty stars* by December 25th.
///
/// Collect stars by solving puzzles. Two puzzles will be made available on each
/// day in the Advent calendar; the second puzzle is unlocked when you complete
/// the first. Each puzzle grants *one star*. Good luck!
///
/// You try to ask why they can't just use a weather machine (<https://adventofcode.com/2015/day/1>) ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet (<https://en.wikipedia.org/wiki/Trebuchet>) ("please hold still, we need to strap you in").
///
/// As they're making the final adjustments, they discover that their
/// calibration document (your puzzle input) has been *amended* by a very young
/// Elf who was apparently just excited to show off her art skills.
/// Consequently, the Elves are having trouble reading the values on the
/// document.
///
/// The newly-improved calibration document consists of lines of text; each line
/// originally contained a specific *calibration value* that the Elves now need
/// to recover. On each line, the calibration value can be found by combining
/// the *first digit* and the *last digit* (in that order) to form a single
/// *two-digit number*.
///
/// For example:
///
/// ```ignore
/// 1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet
/// ```
///
/// In this example, the calibration values of these four lines are `12`, `38`,
/// `15`, and `77`. Adding these together produces `*142*`.
///
/// Consider your entire calibration document. *What is the sum of all of the
/// calibration values?*
pub fn solve_part_1(input: &str) -> Result<u32> {
    let mut nums = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let chars = line.chars();
        let first = first_number(&chars)?;
        let last = last_number(&chars)?;
        nums.push(format!("{}{}", first, last).parse::<u32>()?);
    }

    Ok(nums.iter().sum())
}

const NUMBERS_AS_WORDS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight",
    "nine",
];

/// Convert a line of text like "one2three" to "123".
fn line_to_nums(chars: &std::str::Chars) -> Result<String> {
    let mut nums = String::new();

    let mut chars2 = chars.clone();
    for (index, c) in chars2.by_ref().enumerate() {
        if c.is_ascii_digit() {
            nums.push(c);
        } else {
            let word = NUMBERS_AS_WORDS.iter().position(|&w| w.starts_with(c));
            // If we found the start of a word, make sure we found the whole
            // word.
            if word.is_some() {
                // Get the string.
                let line = chars.as_str();
                // Slice the string based on our current index.
                let line = &line[index..];
                // Check if the lines starts with one of the words.
                if let Some(word) =
                    NUMBERS_AS_WORDS.iter().find(|&w| line.starts_with(w))
                {
                    // If it does, add the number to the list.
                    nums.push_str(match *word {
                        "zero" => "0",
                        "one" => "1",
                        "two" => "2",
                        "three" => "3",
                        "four" => "4",
                        "five" => "5",
                        "six" => "6",
                        "seven" => "7",
                        "eight" => "8",
                        "nine" => "9",
                        _ => anyhow::bail!("Invalid word: {}", word),
                    });
                }
            }
        }
    }

    Ok(nums)
}

/// Your calculation isn't quite right. It looks like some of the digits are
/// actually *spelled out with letters*: `one`, `two`, `three`, `four`, `five`,
/// `six`, `seven`, `eight`, and `nine` *also* count as valid "digits".
///
/// Equipped with this new information, you now need to find the real first and
/// last digit on each line. For example:
///
/// ```ignore
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
/// ```
///
/// In this example, the calibration values are `29`, `83`, `13`, `24`, `42`,
/// `14`, and `76`. Adding these together produces `*281*`.
///
/// *What is the sum of all of the calibration values?*
pub fn solve_part_2(input: &str) -> Result<u32> {
    let mut nums = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let nums_line = line_to_nums(&line.chars())?;
        let chars = nums_line.chars();
        let first = first_number(&chars)?;
        let last = last_number(&chars)?;
        nums.push(format!("{}{}", first, last).parse::<u32>()?);
    }

    Ok(nums.iter().sum())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        // Load the file.
        let input = include_str!("../input/day01.txt");
        assert_eq!(super::solve_part_1(input).unwrap(), 54450);
    }

    #[test]
    fn test_solve_part_2() {
        // Load the file.
        let input = include_str!("../input/day01.txt");
        assert_eq!(super::solve_part_2(input).unwrap(), 54265);
    }
}
