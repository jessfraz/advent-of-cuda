//!  Day 13: Point of Incidence

use anyhow::Result;

/// A terrain data point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Terrain {
    /// Ash.
    Ash,
    /// Rock.
    Rock,
}

impl Terrain {
    /// Parse a terrain data point.
    fn parse(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Self::Ash),
            '#' => Ok(Self::Rock),
            _ => anyhow::bail!("invalid terrain data point: {}", c),
        }
    }
}

/// A terrain row.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TerrainRow(Vec<Terrain>);

impl TerrainRow {
    /// Parse a terrain row.
    fn parse(s: &str) -> Result<Self> {
        let mut row = Vec::with_capacity(s.len());
        for c in s.chars() {
            row.push(Terrain::parse(c)?);
        }
        Ok(Self(row))
    }
}

/// A terrain map.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TerrainMap(Vec<TerrainRow>);

/// A reflection line.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ReflectionLine {
    /// A horizontal line.
    Horizontal((u32, u32)),
    /// A vertical line.
    Vertical((u32, u32)),
}

impl TerrainMap {
    /// Parse a terrain map.
    fn parse(s: &str) -> Result<Self> {
        let mut map = Vec::new();
        for line in s.lines() {
            map.push(TerrainRow::parse(line)?);
        }
        Ok(Self(map))
    }

    /// Find the line of reflection in the map.
    fn find_line_of_reflection(&self) -> Result<ReflectionLine> {
        // Check all the rows.
        for (i, row) in self.0.iter().enumerate() {
            if let Some(next) = self.0.get(i + 1) {
                if row.0 == next.0 {
                    // Found a horizontal line.
                    // Check if all the previous rows are the same.

                    if !is_reflection(self.0.clone(), i) {
                        continue;
                    }

                    return Ok(ReflectionLine::Horizontal((
                        i as u32,
                        (i as u32) + 1,
                    )));
                }
            }
        }

        let columns: Vec<TerrainRow> = (0..self.0[0].0.len())
            .map(|i| self.0.iter().map(|c| c.0[i]).collect())
            .map(TerrainRow)
            .collect();

        // Check all the columns.
        for (i, column) in columns.iter().enumerate() {
            if let Some(next) = columns.get(i + 1) {
                if column == next {
                    // Found a vertical line.
                    // Check if all the previous columns are the same.

                    if !is_reflection(columns.clone(), i) {
                        continue;
                    }

                    return Ok(ReflectionLine::Vertical((
                        i as u32,
                        (i as u32) + 1,
                    )));
                }
            }
        }

        anyhow::bail!("no line of reflection found")
    }
}

/// Check if the previous rows are the same.
fn is_reflection(rows: Vec<TerrainRow>, i: usize) -> bool {
    let mut start = i;
    let mut offset = 2;

    while start > 0 && rows.len() > i + offset {
        start -= 1;
        if rows[start] != rows[i + offset] {
            return false;
        }
        offset += 1;
    }

    true
}

/// With your help, the hot springs team locates an appropriate spring which
/// launches you neatly and precisely up to the edge of *Lava Island*.
///
/// There's just one problem: you don't see any *lava*.
///
/// You *do* see a lot of ash and igneous rock; there are even what look like
/// gray mountains scattered around. After a while, you make your way to a
/// nearby cluster of mountains only to discover that the valley between them is
/// completely full of large *mirrors*. Most of the mirrors seem to be aligned
/// in a consistent way; perhaps you should head in that direction?
///
/// As you move through the valley of mirrors, you find that several of them
/// have fallen from the large metal frames keeping them in place. The mirrors
/// are extremely flat and shiny, and many of the fallen mirrors have lodged
/// into the ash at strange angles. Because the terrain is all one color, it's
/// hard to tell where it's safe to walk or where you're about to run into a
/// mirror.
///
/// You note down the patterns of ash (`.`) and rocks (`#`) that you see as you
/// walk (your puzzle input); perhaps by carefully analyzing these patterns, you
/// can figure out where the mirrors are!
///
/// For example:
///
/// ```ignore
/// #.##..##.
/// ..#.##.#.
/// ##......#
/// ##......#
/// ..#.##.#.
/// ..##..##.
/// #.#.##.#.
///
/// #...##..#
/// #....#..#
/// ..##..###
/// #####.##.
/// #####.##.
/// ..##..###
/// #....#..#
/// ```
///
/// To find the reflection in each pattern, you need to find a perfect
/// reflection across either a horizontal line between two rows or across a
/// vertical line between two columns.
///
/// In the first pattern, the reflection is across a vertical line between two
/// columns; arrows on each of the two columns point at the line between the
/// columns:
///
/// ```ignore
/// 123456789
///     ><
/// #.##..##.
/// ..#.##.#.
/// ##......#
/// ##......#
/// ..#.##.#.
/// ..##..##.
/// #.#.##.#.
///     ><
/// 123456789
/// ```
///
/// In this pattern, the line of reflection is the vertical line between columns
/// 5 and 6. Because the vertical line is not perfectly in the middle of the
/// pattern, part of the pattern (column 1) has nowhere to reflect onto and can
/// be ignored; every other column has a reflected column within the pattern and
/// must match exactly: column 2 matches column 9, column 3 matches 8, 4 matches
/// 7, and 5 matches 6.
///
/// The second pattern reflects across a horizontal line instead:
///
/// ```ignore
/// 1 #...##..# 1
/// 2 #....#..# 2
/// 3 ..##..### 3
/// 4v#####.##.v4
/// 5^#####.##.^5
/// 6 ..##..### 6
/// 7 #....#..# 7
/// ```
///
/// This pattern reflects across the horizontal line between rows 4 and 5. Row 1
/// would reflect with a hypothetical row 8, but since that's not in the
/// pattern, row 1 doesn't need to match anything. The remaining rows match: row
/// 2 matches row 7, row 3 matches row 6, and row 4 matches row 5.
///
/// To *summarize* your pattern notes, add up *the number of columns* to the
/// left of each vertical line of reflection; to that, also add *100 multiplied
/// by the number of rows* above each horizontal line of reflection. In the
/// above example, the first pattern's vertical line has `5` columns to its left
/// and the second pattern's horizontal line has `4` rows above it, a total of
/// `*405*`.
///
/// Find the line of reflection in each of the patterns in your notes. *What
/// number do you get after summarizing all of your notes?*
///
/// To begin, get your puzzle input (<13/input>).
pub fn solve_part_1(input: &str) -> Result<u32> {
    // Split the input ny two new lines.
    let inputs = input.split("\n\n");

    let mut total = 0;
    for map_input in inputs {
        let map = TerrainMap::parse(map_input)?;
        let line = map.find_line_of_reflection()?;

        total += match line {
            ReflectionLine::Horizontal((i, _)) => (i + 1) * 100,
            ReflectionLine::Vertical((i, _)) => i + 1,
        };
    }

    Ok(total)
}

/// Not yet unlocked.
pub fn solve_part_2(_input: &str) -> Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
        assert_eq!(super::solve_part_1(input).unwrap(), 405);

        // Load the file.
        let input = include_str!("../input/day13.txt");
        assert_eq!(super::solve_part_1(input).unwrap(), 27742);
    }

    #[test]
    fn test_solve_part_2() {
        // Load the file.
        let input = include_str!("../input/day13.txt");
        assert_eq!(super::solve_part_2(input).unwrap(), 0);
    }
}
