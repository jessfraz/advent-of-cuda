//!  Day 11: Cosmic Expansion
use anyhow::Result;
use itertools::Itertools;

/// Return the empty lines and columns in the universe.
fn empty_lines_columns(input: &str) -> Result<(Vec<usize>, Vec<usize>)> {
    // Find the lines and columns that contain no galaxies.
    // And expand the universe.
    let lines = input.lines().collect::<Vec<_>>();
    let mut empty_lines = Vec::new();
    let mut columns = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line.chars().all(|c| c == '.') {
            empty_lines.push(i);
        }

        for (j, c) in line.chars().enumerate() {
            if i == 0 {
                columns.push(vec![c]);
            } else {
                columns[j].push(c);
            }
        }
    }

    // Find the columns that contain no galaxies.
    let mut empty_columns = Vec::new();
    for (i, column) in columns.iter().enumerate() {
        if column.iter().all(|c| *c == '.') {
            empty_columns.push(i);
        }
    }

    Ok((empty_lines, empty_columns))
}

/// Expand the universe.
fn expand_universe(input: &str) -> Result<Vec<String>> {
    let (empty_lines, empty_columns) = empty_lines_columns(input)?;

    // Expand the universe.
    let mut expanded: Vec<String> =
        input.lines().map(|s| s.to_string()).collect();
    for (index, i) in empty_lines.iter().enumerate() {
        expanded.insert(
            index + i,
            expanded
                .clone()
                .get(index + i)
                .ok_or_else(|| {
                    anyhow::anyhow!("line does not exist: {}", index + i)
                })?
                .to_string(),
        );
    }
    for (index, i) in empty_columns.iter().enumerate() {
        for (j, line) in expanded.clone().iter().enumerate() {
            let mut chars = line.chars().collect::<Vec<_>>();
            chars.insert(index + i, '.');
            expanded[j] = chars.iter().collect::<String>();
        }
    }

    Ok(expanded)
}

/// You continue following signs for "Hot Springs" and eventually come across
/// an observatory (<https://en.wikipedia.org/wiki/Observatory>). The Elf within
/// turns out to be a researcher studying cosmic expansion using the giant
/// telescope here.
///
/// He doesn't know anything about the missing machine parts; he's only visiting
/// for this research project. However, he confirms that the hot springs are the
/// next-closest area likely to have people; he'll even take you straight there
/// once he's done with today's observation analysis.
///
/// Maybe you can help him with the analysis to speed things up?
///
/// The researcher has collected a bunch of data and compiled the data into a
/// single giant *image* (your puzzle input). The image includes *empty space*
/// (`.`) and *galaxies* (`#`). For example:
///
/// ```ignore
/// ...#......
/// .......#..
/// #.........
/// ..........
/// ......#...
/// .#........
/// .........#
/// ..........
/// .......#..
/// #...#.....
/// ```
///
/// The researcher is trying to figure out the sum of the lengths of the
/// *shortest path between every pair of galaxies*. However, there's a catch:
/// the universe expanded in the time it took the light from those galaxies to
/// reach the observatory.
///
/// Due to something involving gravitational effects, *only some space expands*.
/// In fact, the result is that *any rows or columns that contain no galaxies*
/// should all actually be twice as big.
///
/// In the above example, three columns and two rows contain no galaxies:
///
/// ```ignore
///    v  v  v
///  ...#......
///  .......#..
///  #.........
/// >..........<
///  ......#...
///  .#........
///  .........#
/// >..........<
///  .......#..
///  #...#.....
///    ^  ^  ^
/// ```
///
/// These rows and columns need to be *twice as big*; the result of cosmic
/// expansion therefore looks like this:
///
/// ```ignore
/// ....#........
/// .........#...
/// #............
/// .............
/// .............
/// ........#....
/// .#...........
/// ............#
/// .............
/// .............
/// .........#...
/// #....#.......
/// ```
///
/// Equipped with this expanded universe, the shortest path between every pair
/// of galaxies can be found. It can help to assign every galaxy a unique
/// number:
///
/// ```ignore
/// ....1........
/// .........2...
/// 3............
/// .............
/// .............
/// ........4....
/// .5...........
/// ............6
/// .............
/// .............
/// .........7...
/// 8....9.......
/// ```
///
/// In these 9 galaxies, there are *36 pairs*. Only count each pair once; order
/// within the pair doesn't matter. For each pair, find any shortest path
/// between the two galaxies using only steps that move up, down, left, or right
/// exactly one `.` or `#` at a time. (The shortest path between two galaxies is
/// allowed to pass through another galaxy.)
///
/// For example, here is one of the shortest paths between galaxies `5` and `9`:
///
/// ```ignore
/// ....1........
/// .........2...
/// 3............
/// .............
/// .............
/// ........4....
/// .5...........
/// .##.........6
/// ..##.........
/// ...##........
/// ....##...7...
/// 8....9.......
/// ```
///
/// This path has length `*9*` because it takes a minimum of *nine steps* to get
/// from galaxy `5` to galaxy `9` (the eight locations marked `#` plus the step
/// onto galaxy `9` itself). Here are some other example shortest path lengths:
///
/// * Between galaxy `1` and galaxy `7`: 15
/// * Between galaxy `3` and galaxy `6`: 17
/// * Between galaxy `8` and galaxy `9`: 5
///
/// In this example, after expanding the universe, the sum of the shortest path
/// between all 36 pairs of galaxies is `*374*`.
///
/// Expand the universe, then find the length of the shortest path between every
/// pair of galaxies. *What is the sum of these lengths?*
pub fn solve_part_1(input: &str) -> Result<u64> {
    let expanded = expand_universe(input)?;

    // Get the coordinates of the galaxies.
    let mut galaxies = Vec::new();
    for (i, line) in expanded.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((i, j));
            }
        }
    }

    let mut shortest_distances: Vec<u64> = Vec::new();
    for galaxy_pair in galaxies.iter().combinations(2) {
        let a = galaxy_pair[0];
        let b = galaxy_pair[1];
        // Get the shortest path between the two galaxies.
        let x = if a.0 < b.0 { b.0 - a.0 } else { a.0 - b.0 };
        let y = if a.1 < b.1 { b.1 - a.1 } else { a.1 - b.1 };

        shortest_distances.push((x + y) as u64);
    }

    Ok(shortest_distances.iter().sum())
}

/// The galaxies are much *older* (and thus much *farther apart*) than the
/// researcher initially estimated.
///
/// Now, instead of the expansion you did before, make each empty row or column
/// *one million times* larger. That is, each empty row should be replaced with
/// `1000000` empty rows, and each empty column should be replaced with
/// `1000000` empty columns.
///
/// (In the example above, if each empty row or column were merely `10` times
/// larger, the sum of the shortest paths between every pair of galaxies would
/// be `*1030*`. If each empty row or column were merely `100` times larger, the
/// sum of the shortest paths between every pair of galaxies would be `*8410*`.
/// However, your universe will need to expand far beyond these values.)
///
/// Starting with the same initial image, expand the universe according to these
/// new rules, then find the length of the shortest path between every pair of
/// galaxies. *What is the sum of these lengths?*
pub fn solve_part_2(input: &str, multiplier: u64) -> Result<u64> {
    let (empty_lines, empty_columns) = empty_lines_columns(input)?;
    let lines = input.lines().collect::<Vec<_>>();

    // Get the coordinates of the galaxies.
    let mut galaxies = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((i, j));
            }
        }
    }

    let mut shortest_distances: Vec<u64> = Vec::new();
    for galaxy_pair in galaxies.iter().combinations(2) {
        let a = galaxy_pair[0];
        let b = galaxy_pair[1];
        // Get the shortest path between the two galaxies.
        let x = if a.0 < b.0 { b.0 - a.0 } else { a.0 - b.0 };
        let y = if a.1 < b.1 { b.1 - a.1 } else { a.1 - b.1 };
        let x_range = if a.0 <= b.0 { a.0..=b.0 } else { b.0..=a.0 };
        let y_range = if a.1 <= b.1 { a.1..=b.1 } else { b.1..=a.1 };
        let empty_lines_crossed =
            x_range.filter(|i| empty_lines.contains(i)).count() as u64;
        let empty_columns_crossed =
            y_range.filter(|i| empty_columns.contains(i)).count() as u64;
        let empty_rows = (multiplier * empty_lines_crossed)
            .saturating_sub(empty_lines_crossed);
        let empty_columns = (multiplier * empty_columns_crossed)
            .saturating_sub(empty_columns_crossed);

        shortest_distances.push((x + y) as u64 + empty_rows + empty_columns);
    }

    Ok(shortest_distances.iter().sum())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
        assert_eq!(super::solve_part_1(input).unwrap(), 374);

        // Load the file.
        let input = include_str!("../input/day11.txt");
        assert_eq!(super::solve_part_1(input).unwrap(), 9403026);
    }

    #[test]
    fn test_solve_part_2() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
        assert_eq!(super::solve_part_2(input, 10).unwrap(), 1030);
        assert_eq!(super::solve_part_2(input, 100).unwrap(), 8410);
        // Load the file.
        let input = include_str!("../input/day11.txt");
        assert_eq!(super::solve_part_2(input, 1000000).unwrap(), 543018317006);
    }
}
