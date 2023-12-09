//! This program generates a template for a new day and gets the input for that
//! day.

#![deny(missing_docs)]

use anyhow::Result;

/// The output width of the text.
const OUTPUT_WIDTH: usize = 80;

fn main() -> Result<()> {
    // Parse the day from the command line arguments.
    let args: Vec<String> = std::env::args().collect();

    let day = args
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("No day provided"))?
        .parse::<u32>()?;

    // Get the current working directory.
    let working_dir = std::env::current_dir()?;

    // Get temporary directory.
    let temp_dir = std::env::temp_dir();
    let puzzle_filename = temp_dir.join(format!("day{:02}.md", day));

    let client = aoc_client::AocClient::builder()
        .session_cookie_from_default_locations()?
        .output_width(OUTPUT_WIDTH)?
        .overwrite_files(true)
        .puzzle_filename(&puzzle_filename)
        .year(2023)?
        .day(day)?
        .build()?;

    // Make sure the day is unlocked.
    if !client.day_unlocked() {
        return Err(anyhow::anyhow!("Day {} is not unlocked", day));
    }

    // Get the input for the day.
    get_input(&client, &working_dir, day)?;

    // Generate the template for the day.
    generate_template(&client, &working_dir, day, &puzzle_filename)?;

    Ok(())
}

/// Get the input for the day and save it to our inputs folder.
fn get_input(
    client: &aoc_client::AocClient,
    working_dir: &std::path::Path,
    day: u32,
) -> Result<()> {
    let input = client.get_input()?;

    let input_dir = working_dir.join("input");

    if !input_dir.exists() {
        anyhow::bail!(
            "Input directory does not exist: {}",
            input_dir.display()
        );
    }

    let input_path = input_dir.join(format!("day{:02}.txt", day));

    println!("Writing input to {}", input_path.display());

    std::fs::write(input_path, input.as_bytes())?;

    Ok(())
}

/// Generate the template for the day.
fn generate_template(
    client: &aoc_client::AocClient,
    working_dir: &std::path::Path,
    day: u32,
    puzzle_filename: &std::path::Path,
) -> Result<()> {
    client.save_puzzle_markdown()?;

    // Read the puzzle markdown.
    let puzzle_markdown = std::fs::read_to_string(puzzle_filename)?;
    println!("Puzzle markdown: {}", puzzle_markdown);

    // Clean up the puzzle markdown file.
    std::fs::remove_file(puzzle_filename)?;

    // Parse the puzzle markdown.
    let puzzle = Puzzle::parse(&puzzle_markdown)?;
    println!("Puzzle: {:#?}", puzzle);

    let src_dir = working_dir.join("src");

    if !src_dir.exists() {
        anyhow::bail!("Source directory does not exist: {}", src_dir.display());
    }

    let template_path = src_dir.join(format!("day{:02}.rs", day));

    if template_path.exists() {
        // If the template already exists, let's try and fix the template and
        // add information where we can.

        // Read the existing file.
        let existing = std::fs::read_to_string(&template_path)?;

        // Replace the first line with the title.
        let replaced_title = format!(
            "//!  Day {:02}: {}\n{}",
            day,
            puzzle.title,
            existing
                .lines()
                .skip(1)
                .collect::<Vec<_>>()
                .join("\n")
                .trim()
        );

        let replaced_part_one_text =
            replace_old_comment(&replaced_title, 1, &puzzle.part_one)?;

        let replaced_part_two_text =
            replace_old_comment(&replaced_part_one_text, 2, &puzzle.part_two)?;

        // Write the updated template to the file.
        println!("Updating file at {}", template_path.display());
        std::fs::write(template_path, replaced_part_two_text.as_bytes())?;
    } else {
        // Build the template.
        let template = format!(
            r#"//!  Day {:02}: {}

use anyhow::Result;

/// {}
pub fn solve_part_1(_input: &str) -> Result<u32> {{
    todo!()
}}

/// {}
pub fn solve_part_2(_input: &str) -> Result<u32> {{
    todo!()
}}

#[cfg(test)]
mod tests {{
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {{
        // Load the file.
        let input = include_str!("../input/day{:02}.txt");
        assert_eq!(super::solve_part_1(input).unwrap(), 0);
    }}

    #[test]
    fn test_solve_part_2() {{
        // Load the file.
        let input = include_str!("../input/day{:02}.txt");
        assert_eq!(super::solve_part_2(input).unwrap(), 0);
    }}
}}
"#,
            day,
            puzzle.title,
            puzzle
                .part_one
                .replace('\n', "\n/// ")
                .replace("\n/// \n", "\n///\n"),
            puzzle
                .part_two
                .replace('\n', "\n/// ")
                .replace("\n/// \n", "\n///\n"),
            day,
            day
        );

        // Write the template to the file.
        println!("Writing template to {}", template_path.display());
        std::fs::write(template_path, template.as_bytes())?;
    }

    Ok(())
}

/// Information about the puzzle.
#[derive(Debug)]
struct Puzzle {
    /// The puzzle title.
    title: String,
    /// Part one of the puzzle.
    part_one: String,
    /// Part two of the puzzle.
    part_two: String,
}

impl Puzzle {
    /// Parse the puzzle markdown.
    fn parse(markdown: &str) -> Result<Self> {
        let mut lines = markdown.trim().lines();

        // Get the title from the first line.
        let title = lines
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing title"))?
            .replace('-', "")
            .split(':')
            .nth(1)
            .ok_or_else(|| anyhow::anyhow!("Missing title part after day"))?
            .trim()
            .to_string();

        // Concatenate the rest of the lines.
        let rest = lines
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .trim_matches('-')
            .trim()
            .to_string();
        let parts = rest.split("--- Part Two ---").collect::<Vec<_>>();

        // Get the first part.
        let part_one_raw = parts
            .first()
            .ok_or_else(|| anyhow::anyhow!("Missing part one"))?
            .to_string();

        // Trim everything after "Your puzzle answer was".
        let part_one = trim_answer(&part_one_raw)?;

        // Get the second part.
        let part_two_raw = if let Some(part_two) = parts.get(1) {
            part_two.to_string()
        } else {
            "".to_string()
        };

        // Trim everything after "Your puzzle answer was".
        let part_two = trim_answer(&part_two_raw)?;

        Ok(Self {
            title,
            part_one,
            part_two,
        })
    }
}

fn trim_answer(comment: &str) -> Result<String> {
    let cleaned_comment = comment
        .trim()
        .trim_matches('-')
        .trim()
        .replace("\n\n```\n", "\n\n```ignore\n") // Clean the code samples for rust.
        .replace("\n\n```ignore\n\n", "\n```\n\n") // Clean the code samples for rust.
        .replace("\n*\n", "\n") // Clean up empty bullet points.
        .replace("](/", "](https://adventofcode.com/") // Clean up links to advent of code.
        .to_string();

    let new_comment = cleaned_comment
        .split("Your puzzle answer was")
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing comment"))?
        .trim()
        .to_string();

    let cleaned = new_comment
        .split("Answer:")
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing comment"))?
        .trim()
        .to_string();

    let cleaned = cleaned
        .split("To begin, get your puzzle")
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing comment"))?
        .trim()
        .to_string();

    // Make sure the output width set correctly.
    let markdown = replace_doc_links(&cleaned)?;

    /*Ok(textwrap::wrap(
        &markdown,
        textwrap::Options::new(OUTPUT_WIDTH)
            .word_separator(textwrap::WordSeparator::AsciiSpace),
    )
    .join("\n"))*/
    Ok(markdown)
}

fn replace_old_comment(
    existing: &str,
    part: u32,
    new_comment: &str,
) -> Result<String> {
    // Find the index of the comment.
    let lines = existing.lines().collect::<Vec<_>>();

    // Find the function.
    let fn_index = lines
        .iter()
        .enumerate()
        .find(|(_, line)| {
            line.starts_with(&format!("pub fn solve_part_{}", part))
        })
        .map(|(i, _)| i)
        .ok_or_else(|| anyhow::anyhow!("Missing part {} function", part))?;

    // Get all the lines before the function that start with `///`.
    // Stop when you get to an empty line or a line that doesn't start with
    // `///`.
    let comments = lines
        .iter()
        .take(fn_index)
        .rev()
        .take_while(|line| line.starts_with("///"))
        .map(|line| line.trim_start_matches("///").trim())
        .collect::<Vec<_>>();

    // Find the index of the comment.
    let comment = comments
        .last() // Since we reversed the lines, the last comment is the first one.
        .ok_or_else(|| anyhow::anyhow!("Missing comment"))?;
    let comment_index = lines
        .iter()
        .enumerate()
        .find(|(_, line)| line.contains(comment))
        .map(|(i, _)| i)
        .ok_or_else(|| anyhow::anyhow!("Missing comment index"))?;

    // Delete everything from the comment to the function.
    let mut lines = existing.lines().collect::<Vec<_>>();
    lines.drain(comment_index..fn_index);

    // Insert the new comments at the comment index.
    let part_comment = if new_comment.is_empty() {
        "/// Not yet unlocked.".to_string()
    } else {
        format!(
            "/// {}",
            new_comment
                .replace('\n', "\n/// ")
                .replace("\n/// \n", "\n///\n")
        )
    };
    lines.insert(comment_index, &part_comment);

    Ok(lines.join("\n"))
}

/// Replace the markdown links with doc links.
fn replace_doc_links(comment: &str) -> Result<String> {
    let re = regex::Regex::new(r"\[([^]]+)]\(([^)]+)\)")?;

    let output = re.replace_all(comment, "$1 (<$2>)");

    Ok(output.to_string())
}
