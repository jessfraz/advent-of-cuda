//!  Day 09: Mirage Maintenance
use anyhow::Result;

/// Get the history of the line.
fn get_history(line: &[i32]) -> Vec<i32> {
    let mut history = Vec::new();
    for i in 0..line.len() - 1 {
        history.push(line[i + 1] - line[i]);
    }

    history
}

/// Get the next value of the line by iterating over the histories.
fn get_next_value(history: &[i32]) -> Result<i32> {
    let mut histories = Vec::new();
    let mut history = history.to_vec();
    histories.push(history.clone());

    while history.iter().any(|&x| x != 0) {
        history = get_history(&history);
        histories.push(history.clone());
    }

    let mut value = 0;
    for history in histories.iter().rev() {
        let last = history
            .last()
            .ok_or_else(|| anyhow::anyhow!("no last item"))?;
        value += last;
    }

    Ok(value)
}

/// Get the first value of the line by iterating over the histories.
fn get_first_value(history: &[i32]) -> Result<i32> {
    let mut histories = Vec::new();
    let mut history = history.to_vec();
    histories.push(history.clone());

    while history.iter().any(|&x| x != 0) {
        history = get_history(&history);
        histories.push(history.clone());
    }

    let mut value = 0;
    for history in histories.iter().rev() {
        let first = history
            .first()
            .ok_or_else(|| anyhow::anyhow!("no first item"))?;
        value = first - value;
    }

    Ok(value)
}

/// You ride the camel through the sandstorm and stop where the ghost's maps
/// told you to stop. The sandstorm subsequently subsides, somehow seeing you
/// standing at an *oasis*!
///
/// The camel goes to get some water and you stretch your neck. As you look up,
/// you discover what must be yet another giant floating island, this one made
/// of metal! That must be where the *parts to fix the sand machines* come from.
///
/// There's even a hang glider (<https://en.wikipedia.org/wiki/Hang_gliding>)
/// partially buried in the sand here; once the sun rises and heats up the sand,
/// you might be able to use the glider and the hot air to get all the way up to
/// the metal island!
///
/// While you wait for the sun to rise, you admire the oasis hidden here in the
/// middle of Desert Island. It must have a delicate ecosystem; you might as
/// well take some ecological readings while you wait. Maybe you can report any
/// environmental instabilities you find to someone so the oasis can be around
/// for the next sandstorm-worn traveler.
///
/// You pull out your handy *Oasis And Sand Instability Sensor* and analyze your
/// surroundings. The OASIS produces a report of many values and how they are
/// changing over time (your puzzle input). Each line in the report contains the
/// *history* of a single value. For example:
///
/// ```ignore
/// 0 3 6 9 12 15
/// 1 3 6 10 15 21
/// 10 13 16 21 30 45
/// ```
///
/// To best protect the oasis, your environmental report should include a
/// *prediction of the next value* in each history. To do this, start by making
/// a new sequence from the *difference at each step* of your history. If that
/// sequence is *not* all zeroes, repeat this process, using the sequence you
/// just generated as the input sequence. Once all of the values in your latest
/// sequence are zeroes, you can extrapolate what the next value of the original
/// history should be.
///
/// In the above dataset, the first history is `0 3 6 9 12 15`. Because the
/// values increase by `3` each step, the first sequence of differences that you
/// generate will be `3 3 3 3 3`. Note that this sequence has one fewer value
/// than the input sequence because at each step it considers two numbers from
/// the input. Since these values aren't *all zero*, repeat the process: the
/// values differ by `0` at each step, so the next sequence is `0 0 0 0`. This
/// means you have enough information to extrapolate the history! Visually,
/// these sequences can be arranged like this:
///
/// ```ignore
/// 0   3   6   9  12  15
///   3   3   3   3   3
///     0   0   0   0
/// ```
///
/// To extrapolate, start by adding a new zero to the end of your list of
/// zeroes; because the zeroes represent differences between the two values
/// above them, this also means there is now a placeholder in every sequence
/// above it:
///
/// ```ignore
/// 0   3   6   9  12  15   B
///   3   3   3   3   3   A
///     0   0   0   0   0
/// ```
///
/// You can then start filling in placeholders from the bottom up. `A` needs to
/// be the result of increasing `3` (the value to its left) by `0` (the value
/// below it); this means `A` must be `*3*`:
///
/// ```ignore
/// 0   3   6   9  12  15   B
///   3   3   3   3   3   3
///     0   0   0   0   0
/// ```
///
/// Finally, you can fill in `B`, which needs to be the result of increasing
/// `15` (the value to its left) by `3` (the value below it), or `*18*`:
///
/// ```ignore
/// 0   3   6   9  12  15  18
///   3   3   3   3   3   3
///     0   0   0   0   0
/// ```
///
/// So, the next value of the first history is `*18*`.
///
/// Finding all-zero differences for the second history requires an additional
/// sequence:
///
/// ```ignore
/// 1   3   6  10  15  21
///   2   3   4   5   6
///     1   1   1   1
///       0   0   0
/// ```
///
/// Then, following the same process as before, work out the next value in each
/// sequence from the bottom up:
///
/// ```ignore
/// 1   3   6  10  15  21  28
///   2   3   4   5   6   7
///     1   1   1   1   1
///       0   0   0   0
/// ```
///
/// So, the next value of the second history is `*28*`.
///
/// The third history requires even more sequences, but its next value can be
/// found the same way:
///
/// ```ignore
/// 10  13  16  21  30  45  68
///    3   3   5   9  15  23
///      0   2   4   6   8
///        2   2   2   2
///          0   0   0
/// ```
///
/// So, the next value of the third history is `*68*`.
///
/// If you find the next value for each history in this example and add them
/// together, you get `*114*`.
///
/// Analyze your OASIS report and extrapolate the next value for each history.
/// *What is the sum of these extrapolated values?*
pub fn solve_part_1(input: &str) -> Result<u32> {
    // Get the vector of numbers in each line from the input.
    let lines = input
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<_>>();
            split
                .iter()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut next_values = Vec::new();
    for line in lines {
        // Get the history of the line.
        let next_value = get_next_value(&line);
        next_values.push(next_value?);
    }

    Ok(next_values.iter().sum::<i32>() as u32)
}

/// Of course, it would be nice to have *even more history* included in your
/// report. Surely it's safe to just *extrapolate backwards* as well, right?
///
/// For each history, repeat the process of finding differences until the
/// sequence of differences is entirely zero. Then, rather than adding a zero to
/// the end and filling in the next values of each previous sequence, you should
/// instead add a zero to the *beginning* of your sequence of zeroes, then fill
/// in new *first* values for each previous sequence.
///
/// In particular, here is what the third example history looks like when
/// extrapolating back in time:
///
/// ```ignore
/// 5  10  13  16  21  30  45
///   5   3   3   5   9  15
///    -2   0   2   4   6
///       2   2   2   2
///         0   0   0
/// ```
///
/// Adding the new values on the left side of each sequence from bottom to top
/// eventually reveals the new left-most history value: `*5*`.
///
/// Doing this for the remaining example data above results in previous values
/// of `*-3*` for the first history and `*0*` for the second history. Adding all
/// three new values together produces `*2*`.
///
/// Analyze your OASIS report again, this time extrapolating the *previous*
/// value for each history. *What is the sum of these extrapolated values?*
pub fn solve_part_2(input: &str) -> Result<i32> {
    // Get the vector of numbers in each line from the input.
    let lines = input
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<_>>();
            split
                .iter()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut first_values = Vec::new();
    for line in lines {
        // Get the history of the line.
        let first_value = get_first_value(&line);
        first_values.push(first_value?);
    }

    Ok(first_values.iter().sum::<i32>())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        assert_eq!(super::solve_part_1(input).unwrap(), 114);

        // Load the file.
        let input = include_str!("../input/day09.txt");
        assert_eq!(super::solve_part_1(input).unwrap(), 1969958987);
    }

    #[test]
    fn test_solve_part_2() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        assert_eq!(super::solve_part_2(input).unwrap(), 2);

        // Load the file.
        let input = include_str!("../input/day09.txt");
        assert_eq!(super::solve_part_2(input).unwrap(), 1068);
    }
}
