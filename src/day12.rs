//!  Day 12: Hot Springs
use anyhow::Result;
use itertools::Itertools;
use rayon::prelude::*;

/// Spring data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    /// Operational spring.
    Operational,
    /// Broken spring.
    Broken,
    /// Unknown spring.
    Unknown,
}

impl Spring {
    /// Parse a spring from a character.
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Broken,
            '?' => Self::Unknown,
            _ => panic!("invalid spring character: {}", c),
        }
    }
}

/// Spring row.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SpringRow {
    /// Springs.
    springs: Vec<Spring>,
    /// Groups of operational springs.
    groups: Vec<usize>,
}

impl SpringRow {
    /// Get the number of possible arrangements of operational and broken
    /// springs that meet the given criteria.
    /// For example:
    /// * `???.### 1,1,3` - `*1*` arrangement (the first three unknown springs
    ///  must be broken, then operational, then broken (`#.#`), making the
    ///  whole row `#.#.###`).
    fn arrangements(&self) -> Result<u32> {
        if let Some(index) = self
            .springs
            .par_iter()
            .position_any(|spring| *spring == Spring::Unknown)
        {
            // If we have an unknown spring, we need to try both operational
            // and broken and add the results together.

            // Treat unknown spring as damaged.
            let mut as_damaged_spring = self.springs.clone();
            as_damaged_spring[index] = Spring::Broken;
            let as_damaged = SpringRow {
                springs: as_damaged_spring,
                groups: self.groups.clone(),
            };

            // Treat unknown spring as operational.
            let mut as_operational_spring = self.springs.clone();
            as_operational_spring[index] = Spring::Operational;
            let as_operational = SpringRow {
                springs: as_operational_spring,
                groups: self.groups.clone(),
            };

            Ok(as_damaged.arrangements()? + as_operational.arrangements()?)
        } else if self.is_valid() {
            // Check if we are valid.
            Ok(1)
        } else {
            // Not valid.
            Ok(0)
        }
    }

    /// Check if the spring row is valid with the number of groups.
    fn is_valid(&self) -> bool {
        self.springs
            .iter()
            .group_by(|item| *item)
            .into_iter()
            .filter_map(|(key, group)| {
                if *key == Spring::Broken {
                    Some(group.count())
                } else {
                    None
                }
            })
            .eq(self.groups.iter().copied())
    }

    /// Parse a row of springs.
    fn parse(s: &str) -> Result<SpringRow> {
        // Split on whitespace.
        let mut parts = s.split_whitespace();
        let springs_str = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing springs"))?;

        // Parse the springs.
        let springs = springs_str.chars().map(Spring::parse).collect();

        // Parse the groups.
        let groups = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing groups"))?
            .split(',')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(SpringRow { springs, groups })
    }

    /// Parse a row of springs, copied 5 times.
    fn parse_part_2(s: &str) -> Result<SpringRow> {
        let mut row = SpringRow::parse(s)?;

        // Replace the list of spring conditions with five copies of itself (separated by ?).
        row.springs = row
            .springs
            .iter()
            .copied()
            .chain([Spring::Unknown])
            .cycle()
            .take(row.springs.len() * 5 + 4)
            .collect();
        row.groups = row
            .groups
            .iter()
            .copied()
            .cycle()
            .take(row.groups.len() * 5)
            .collect();

        Ok(row)
    }
}

fn count_possible_arangements(
    mut springs: Vec<Spring>,
    groups: Vec<usize>,
) -> Result<u64> {
    // To make the Broken recursion case simpler.
    springs.push(Spring::Operational);
    let mut cache = vec![vec![None; springs.len()]; groups.len()];
    count_possible_arangements_inner(&springs, &groups, &mut cache)
}

fn count_possible_arangements_inner(
    springs: &[Spring],
    groups: &[usize],
    cache: &mut [Vec<Option<u64>>],
) -> Result<u64> {
    if groups.is_empty() {
        return if springs.contains(&Spring::Broken) {
            // Too many previous unknowns were counted as broken.
            Ok(0)
        } else {
            // All remaining unknowns are operational.
            Ok(1)
        };
    }
    if springs.len() < groups.iter().sum::<usize>() + groups.len() {
        // Not enough space for remaining numbers.
        return Ok(0);
    }
    if let Some(cached) = cache[groups.len() - 1][springs.len() - 1] {
        return Ok(cached);
    }
    let mut arangements = 0;
    if springs[0] != Spring::Broken {
        // Assume operational.
        arangements +=
            count_possible_arangements_inner(&springs[1..], groups, cache)?;
    }
    let next_group_size = groups[0];
    if !springs[..next_group_size].contains(&Spring::Operational)
        && springs[next_group_size] != Spring::Broken
    {
        // Assume broken.
        arangements += count_possible_arangements_inner(
            &springs[next_group_size + 1..],
            &groups[1..],
            cache,
        )?;
    }
    cache[groups.len() - 1][springs.len() - 1] = Some(arangements);
    Ok(arangements)
}

/// You finally reach the hot springs! You can see steam rising from secluded
/// areas attached to the primary, ornate building.
///
/// As you turn to enter, the researcher (<11>) stops you. "Wait - I thought you
/// were looking for the hot springs, weren't you?" You indicate that this
/// definitely looks like hot springs to you.
///
/// "Oh, sorry, common mistake! This is actually the onsen
/// (<https://en.wikipedia.org/wiki/Onsen>)! The hot springs are next door."
///
/// You look in the direction the researcher is pointing and suddenly notice the
/// massive metal helixes towering overhead. "This way!"
///
/// It only takes you a few more steps to reach the main gate of the massive
/// fenced-off area containing the springs. You go through the gate and into a
/// small administrative building.
///
/// "Hello! What brings you to the hot springs today? Sorry they're not very hot
/// right now; we're having a *lava shortage* at the moment." You ask about the
/// missing machine parts for Desert Island.
///
/// "Oh, all of Gear Island is currently offline! Nothing is being manufactured
/// at the moment, not until we get more lava to heat our forges. And our
/// springs. The springs aren't very springy unless they're hot!"
///
/// "Say, could you go up and see why the lava stopped flowing? The springs are
/// too cold for normal operation, but we should be able to find one springy
/// enough to launch *you* up there!"
///
/// There's just one problem - many of the springs have fallen into disrepair,
/// so they're not actually sure which springs would even be *safe* to use!
/// Worse yet, their *condition records of which springs are damaged* (your
/// puzzle input) are also damaged! You'll need to help them repair the damaged
/// records.
///
/// In the giant field just outside, the springs are arranged into *rows*. For
/// each row, the condition records show every spring and whether it is
/// *operational* (`.`) or *damaged* (`#`). This is the part of the condition
/// records that is itself damaged; for some springs, it is simply *unknown*
/// (`?`) whether the spring is operational or damaged.
///
/// However, the engineer that produced the condition records also duplicated
/// some of this information in a different format! After the list of springs
/// for a given row, the size of each *contiguous group of damaged springs* is
/// listed in the order those groups appear in the row. This list always
/// acgroups for every damaged spring, and each number is the entire size of its
/// contiguous group (that is, groups are always separated by at least one
/// operational spring: `####` would always be `4`, never `2,2`).
///
/// So, condition records with no unknown spring conditions might look like
/// this:
///
/// ```ignore
/// #.#.### 1,1,3
/// .#...#....###. 1,1,3
/// .#.###.#.###### 1,3,1,6
/// ####.#...#... 4,1,1
/// #....######..#####. 1,6,5
/// .###.##....# 3,2,1
/// ```
///
/// However, the condition records are partially damaged; some of the springs'
/// conditions are actually *unknown* (`?`). For example:
///
/// ```ignore
/// ???.### 1,1,3
/// .??..??...?##. 1,1,3
/// ?#?#?#?#?#?#?#? 1,3,1,6
/// ????.#...#... 4,1,1
/// ????.######..#####. 1,6,5
/// ?###???????? 3,2,1
/// ```
///
/// Equipped with this information, it is your job to figure out *how many
/// different arrangements* of operational and broken springs fit the given
/// criteria in each row.
///
/// In the first line (`???.### 1,1,3`), there is exactly *one* way separate
/// groups of one, one, and three broken springs (in that order) can appear in
/// that row: the first three unknown springs must be broken, then operational,
/// then broken (`#.#`), making the whole row `#.#.###`.
///
/// The second line is more interesting: `.??..??...?##. 1,1,3` could be a total
/// of *four* different arrangements. The last `?` must always be broken (to
/// satisfy the final contiguous group of three broken springs), and each `??`
/// must hide exactly one of the two broken springs. (Neither `??` could be both
/// broken springs or they would form a single contiguous group of two; if that
/// were true, the numbers afterward would have been `2,3` instead.) Since each
/// `??` can either be `#.` or `.#`, there are four possible arrangements of
/// springs.
///
/// The last line is actually consistent with *ten* different arrangements!
/// Because the first number is `3`, the first and second `?` must both be `.`
/// (if either were `#`, the first number would have to be `4` or higher).
/// However, the remaining run of unknown spring conditions have many different
/// ways they could hold groups of two and one broken springs:
///
/// ```ignore
/// ?###???????? 3,2,1
/// .###.##.#...
/// .###.##..#..
/// .###.##...#.
/// .###.##....#
/// .###..##.#..
/// .###..##..#.
/// .###..##...#
/// .###...##.#.
/// .###...##..#
/// .###....##.#
/// ```
///
/// In this example, the number of possible arrangements for each row is:
///
/// * `???.### 1,1,3` - `*1*` arrangement
/// * `.??..??...?##. 1,1,3` - `*4*` arrangements
/// * `?#?#?#?#?#?#?#? 1,3,1,6` - `*1*` arrangement
/// * `????.#...#... 4,1,1` - `*1*` arrangement
/// * `????.######..#####. 1,6,5` - `*4*` arrangements
/// * `?###???????? 3,2,1` - `*10*` arrangements
///
/// Adding all of the possible arrangement groups together produces a total of
/// `*21*` arrangements.
///
/// For each row, count all of the different arrangements of operational and
/// broken springs that meet the given criteria. *What is the sum of those
/// groups?*
pub fn solve_part_1(input: &str) -> Result<u32> {
    let spring_rows = input
        .lines()
        .map(|l| SpringRow::parse(l).unwrap())
        .collect::<Vec<_>>();

    let arrangements = spring_rows
        .par_iter()
        .map(|r| r.arrangements().unwrap())
        .collect::<Vec<u32>>();

    Ok(arrangements.par_iter().sum())
}

/// As you look out at the field of springs, you feel like there are way more
/// springs than the condition records list. When you examine the records, you
/// discover that they were actually *folded up* this whole time!
///
/// To *unfold the records*, on each row, replace the list of spring conditions
/// with five copies of itself (separated by `?`) and replace the list of
/// contiguous groups of damaged springs with five copies of itself (separated
/// by `,`).
///
/// So, this row:
///
/// ```ignore
/// .# 1
/// ```
///
/// Would become:
///
/// ```ignore
/// .#?.#?.#?.#?.# 1,1,1,1,1
/// ```
///
/// The first line of the above example would become:
///
/// ```ignore
/// ???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3
/// ```
///
/// In the above example, after unfolding, the number of possible arrangements
/// for some rows is now much larger:
///
/// * `???.### 1,1,3` - `*1*` arrangement
/// * `.??..??...?##. 1,1,3` - `*16384*` arrangements
/// * `?#?#?#?#?#?#?#? 1,3,1,6` - `*1*` arrangement
/// * `????.#...#... 4,1,1` - `*16*` arrangements
/// * `????.######..#####. 1,6,5` - `*2500*` arrangements
/// * `?###???????? 3,2,1` - `*506250*` arrangements
///
/// After unfolding, adding all of the possible arrangement groups together
/// produces `*525152*`.
///
/// Unfold your condition records; *what is the new sum of possible arrangement
/// groups?*
pub fn solve_part_2(input: &str) -> Result<u64> {
    let spring_rows = input
        .lines()
        .map(|l| SpringRow::parse_part_2(l).unwrap())
        .collect::<Vec<_>>();

    let mut arrangements = Vec::new();
    for row in &spring_rows {
        let a = count_possible_arangements(
            row.springs.clone(),
            row.groups.clone(),
        )?;
        arrangements.push(a);
    }

    Ok(arrangements.par_iter().sum())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
        assert_eq!(super::solve_part_1(input).unwrap(), 21);

        // Load the file.
        let input = include_str!("../input/day12.txt");
        assert_eq!(super::solve_part_1(input).unwrap(), 8270);
    }

    #[test]
    fn test_solve_part_2() {
        let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
        assert_eq!(super::solve_part_2(input).unwrap(), 525152);

        // Load the file.
        let input = include_str!("../input/day12.txt");
        assert_eq!(super::solve_part_2(input).unwrap(), 0);
    }
}
