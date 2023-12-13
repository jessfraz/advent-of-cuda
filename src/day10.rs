//!  Day 10: Pipe Maze
use anyhow::Result;

/// The direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    SouthNorth,
    EastWest,
}

/// The coordinates.
type Coordinates = [i32; 2];

/// The tile.
#[derive(Debug, Clone, Copy)]
struct Tile {
    direction: Direction,
    gates: [Coordinates; 2],
    is_edge: bool,
    is_loop: bool,
    is_start: bool,
    position: Coordinates,
}

impl Tile {
    fn new(
        gates: [Coordinates; 2],
        position: Coordinates,
        direction: Direction,
    ) -> Tile {
        Tile {
            direction,
            gates,
            is_edge: false,
            is_loop: false,
            is_start: false,
            position,
        }
    }

    fn start_tile(position: Coordinates) -> Tile {
        Tile {
            direction: Direction::NorthWest, // By default, start top left.
            gates: [[0; 2], [0; 2]],
            is_edge: false,
            is_loop: true,
            is_start: true,
            position,
        }
    }

    fn to(&self, from: Coordinates) -> Coordinates {
        if self.gates[0] == from {
            self.gates[1]
        } else {
            self.gates[0]
        }
    }
}

#[derive(Debug, Clone)]
struct Maze {
    start: Tile,
    tiles: Vec<Vec<Tile>>,
}

impl Maze {
    fn parse(input: Vec<&str>) -> Result<Maze> {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        let mut maze = Maze {
            start: Tile::new(
                Default::default(),
                Default::default(),
                Direction::NorthWest,
            ),
            tiles: Vec::new(),
        };
        for (y, line) in input.iter().enumerate() {
            let mut row: Vec<Tile> = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let mut tile = match c {
                    '-' => Tile::new(
                        [[x as i32 - 1, y as i32], [x as i32 + 1, y as i32]],
                        [x as i32, y as i32],
                        Direction::EastWest,
                    ),
                    '.' => Tile::new(
                        Default::default(),
                        [x as i32, y as i32],
                        Direction::NorthWest,
                    ),
                    '7' => Tile::new(
                        [[x as i32, y as i32 + 1], [x as i32 - 1, y as i32]],
                        [x as i32, y as i32],
                        Direction::SouthWest,
                    ),
                    'F' => Tile::new(
                        [[x as i32, y as i32 + 1], [x as i32 + 1, y as i32]],
                        [x as i32, y as i32],
                        Direction::SouthEast,
                    ),
                    'J' => Tile::new(
                        [[x as i32, y as i32 - 1], [x as i32 - 1, y as i32]],
                        [x as i32, y as i32],
                        Direction::NorthWest,
                    ),
                    'L' => Tile::new(
                        [[x as i32, y as i32 - 1], [x as i32 + 1, y as i32]],
                        [x as i32, y as i32],
                        Direction::NorthEast,
                    ),
                    'S' => Tile::start_tile([x as i32, y as i32]),
                    '|' => Tile::new(
                        [[x as i32, y as i32 - 1], [x as i32, y as i32 + 1]],
                        [x as i32, y as i32],
                        Direction::SouthNorth,
                    ),
                    _ => anyhow::bail!("Invalid character: {}", c),
                };
                if tile.is_start {
                    maze.start = tile;
                }
                if x == 0
                    || x == line.len() - 1
                    || y == 0
                    || y == input.len() - 1
                {
                    tile.is_edge = true;
                }
                row.push(tile);
            }
            tiles.push(row);
        }
        maze.tiles = tiles;
        maze.find_start_gates()?;
        Ok(maze)
    }

    fn find_start_gates(&mut self) -> Result<()> {
        // Define the ranges around the start tile
        let x_range = [
            (self.start.position[0] - 1).max(0),
            (self.start.position[0] + 1).min(self.tiles[0].len() as i32 - 1),
        ];
        let y_range = [
            (self.start.position[1] - 1).max(0),
            (self.start.position[1] + 1).min(self.tiles.len() as i32 - 1),
        ];

        let mut entries: Vec<Coordinates> = Vec::new();

        // Enumerate over every tile in the tiles Vec that is within our defined
        // range
        for i in y_range[0]..=y_range[1] {
            for j in x_range[0]..=x_range[1] {
                // Skip the start tile itself
                if i == self.start.position[1] && j == self.start.position[0] {
                    continue;
                }
                // Check if the tile's list of entries contains the start
                // position
                if self.tiles[i as usize][j as usize]
                    .gates
                    .contains(&self.start.position)
                {
                    entries.push([j, i]);
                }
            }
        }

        entries.sort();

        // Update start entries only if entries vec has enough items
        if entries.len() > 1 {
            self.start.gates[0] = entries[0];
            self.start.gates[1] = entries[1];
        }

        // Update start tile direction
        let direction = match (self.start.gates[0], self.start.gates[1]) {
            ([x, y], [x2, y2]) if x == x2 && y < y2 => Direction::SouthNorth,
            ([x, y], [x2, y2]) if x == x2 && y > y2 => Direction::SouthNorth,
            ([x, y], [x2, y2]) if x < x2 && y == y2 => Direction::EastWest,
            ([x, y], [x2, y2])
                if (x < x2 && y > y2) && (self.start.position[1] < y) =>
            {
                Direction::SouthEast
            }
            ([x, y], [x2, y2])
                if (x < x2 && y < y2) && (self.start.position[0] > x) =>
            {
                Direction::SouthWest
            }
            ([x, y], [x2, y2]) if x < x2 && y > y2 => Direction::NorthWest,
            ([x, y], [x2, y2]) if x < x2 && y < y2 => Direction::NorthEast,
            _ => Direction::NorthWest,
        };
        self.start.direction = direction;
        self.tiles[self.start.position[1] as usize]
            [self.start.position[0] as usize]
            .direction = direction;

        Ok(())
    }

    fn walk_loop(&mut self) -> Result<Vec<Coordinates>> {
        let mut loop_tiles = Vec::new();
        let mut next = self.start.gates[0];
        loop_tiles.push(next);
        let mut prev = self.start.position;

        while next != self.start.position {
            let tile = &mut self.tiles[next[1] as usize][next[0] as usize];
            tile.is_loop = true;
            let cur = next;
            next = tile.to(prev);
            prev = cur;
            loop_tiles.push(next);
        }
        Ok(loop_tiles)
    }

    fn ray_cast_tile(&self, from: Coordinates) -> Result<i32> {
        let mut count = 0;
        for i in 0..from[0] {
            let tile = &self.tiles[from[1] as usize][i as usize];
            if tile.is_loop
                && (tile.direction == Direction::SouthNorth
                    || tile.direction == Direction::SouthWest
                    || tile.direction == Direction::SouthEast)
            {
                count += 1;
            }
        }
        if count % 2 == 0 {
            Ok(0)
        } else {
            Ok(1)
        }
    }

    fn count_enclosed(&self) -> Result<u32> {
        let mut count = 0;
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if !tile.is_loop && !tile.is_edge {
                    count += self.ray_cast_tile([x as i32, y as i32])?;
                }
            }
        }
        Ok(count as u32)
    }
}

/// You use the hang glider to ride the hot air from Desert Island all the way
/// up to the floating metal island. This island is surprisingly cold and there
/// definitely aren't any thermals to glide on, so you leave your hang glider
/// behind.
///
/// You wander around for a while, but you don't find any people or animals.
/// However, you do occasionally find signposts labeled "Hot Springs
/// (<https://en.wikipedia.org/wiki/Hot_spring>)" pointing in a seemingly
/// consistent direction; maybe you can find someone at the hot springs and ask
/// them where the desert-machine parts are made.
///
/// The landscape here is alien; even the flowers and trees are made of metal.
/// As you stop to admire some metal grass, you notice something metallic scurry
/// away in your peripheral vision and jump into a big pipe! It didn't look like
/// any animal you've ever seen; if you want a better look, you'll need to get
/// ahead of it.
///
/// Scanning the area, you discover that the entire field you're standing on is
/// densely packed with pipes; it was hard to tell at first because they're the
/// same metallic silver color as the "ground". You make a quick sketch of all
/// of the surface pipes you can see (your puzzle input).
///
/// The pipes are arranged in a two-dimensional grid of *tiles*:
///
/// * `|` is a *vertical pipe* connecting north and south.
/// * `-` is a *horizontal pipe* connecting east and west.
/// * `L` is a *90-degree bend* connecting north and east.
/// * `J` is a *90-degree bend* connecting north and west.
/// * `7` is a *90-degree bend* connecting south and west.
/// * `F` is a *90-degree bend* connecting south and east.
/// * `.` is *ground*; there is no pipe in this tile.
/// * `S` is the *starting position* of the animal; there is a pipe on this
///   tile, but your sketch doesn't show what shape the pipe has.
///
/// Based on the acoustics of the animal's scurrying, you're confident the pipe
/// that contains the animal is *one large, continuous loop*.
///
/// For example, here is a square loop of pipe:
///
/// ```ignore
/// .....
/// .F-7.
/// .|.|.
/// .L-J.
/// .....
/// ```
///
/// If the animal had entered this loop in the northwest corner, the sketch
/// would instead look like this:
///
/// ```ignore
/// .....
/// .S-7.
/// .|.|.
/// .L-J.
/// .....
/// ```
///
/// In the above diagram, the `S` tile is still a 90-degree `F` bend: you can
/// tell because of how the adjacent pipes connect to it.
///
/// Unfortunately, there are also many pipes that *aren't connected to the
/// loop*! This sketch shows the same loop as above:
///
/// ```ignore
/// -L|F7
/// 7S-7|
/// L|7||
/// -L-J|
/// L|-JF
/// ```
///
/// In the above diagram, you can still figure out which pipes form the main
/// loop: they're the ones connected to `S`, pipes those pipes connect to, pipes
/// *those* pipes connect to, and so on. Every pipe in the main loop connects to
/// its two neighbors (including `S`, which will have exactly two pipes
/// connecting to it, and which is assumed to connect back to those two pipes).
///
/// Here is a sketch that contains a slightly more complex main loop:
///
/// ```ignore
/// ..F7.
/// .FJ|.
/// SJ.L7
/// |F--J
/// LJ...
/// ```
///
/// Here's the same example sketch with the extra, non-main-loop pipe tiles also
/// shown:
///
/// ```ignore
/// 7-F7-
/// .FJ|7
/// SJLL7
/// |F--J
/// LJ.LJ
/// ```
///
/// If you want to *get out ahead of the animal*, you should find the tile in
/// the loop that is *farthest* from the starting position. Because the animal
/// is in the pipe, it doesn't make sense to measure this by direct distance.
/// Instead, you need to find the tile that would take the longest number of
/// steps *along the loop* to reach from the starting point - regardless of
/// which way around the loop the animal went.
///
/// In the first example with the square loop:
///
/// ```ignore
/// .....
/// .S-7.
/// .|.|.
/// .L-J.
/// .....
/// ```
///
/// You can count the distance each tile in the loop is from the starting point
/// like this:
///
/// ```ignore
/// .....
/// .012.
/// .1.3.
/// .234.
/// .....
/// ```
///
/// In this example, the farthest point from the start is `*4*` steps away.
///
/// Here's the more complex loop again:
///
/// ```ignore
/// ..F7.
/// .FJ|.
/// SJ.L7
/// |F--J
/// LJ...
/// ```
///
/// Here are the distances for each tile on that loop:
///
/// ```ignore
/// ..45.
/// .236.
/// 01.78
/// 14567
/// 23...
/// ```
///
/// Find the single giant loop starting at `S`. *How many steps along the loop
/// does it take to get from the starting position to the point farthest from
/// the starting position?*
pub fn solve_part_1(input: &str) -> Result<u32> {
    let lines = input.lines().collect();
    let mut maze = Maze::parse(lines)?;
    let loop_tiles = maze.walk_loop()?;
    Ok((loop_tiles.len() as u32) / 2)
}

/// You quickly reach the farthest point of the loop, but the animal never
/// emerges. Maybe its nest is *within the area enclosed by the loop*?
///
/// To determine whether it's even worth taking the time to search for such a
/// nest, you should calculate how many tiles are contained within the loop. For
/// example:
///
/// ```ignore
/// ...........
/// .S-------7.
/// .|F-----7|.
/// .||.....||.
/// .||.....||.
/// .|L-7.F-J|.
/// .|..|.|..|.
/// .L--J.L--J.
/// ...........
/// ```
///
/// The above loop encloses merely *four tiles* - the two pairs of `.` in the
/// southwest and southeast (marked `I` below). The middle `.` tiles (marked `O`
/// below) are *not* in the loop. Here is the same loop again with those regions
/// marked:
///
/// ```ignore
/// ...........
/// .S-------7.
/// .|F-----7|.
/// .||OOOOO||.
/// .||OOOOO||.
/// .|L-7OF-J|.
/// .|II|O|II|.
/// .L--JOL--J.
/// .....O.....
/// ```
///
/// In fact, there doesn't even need to be a full tile path to the outside for
/// tiles to count as outside the loop - squeezing between pipes is also
/// allowed! Here, `I` is still within the loop and `O` is still outside the
/// loop:
///
/// ```ignore
/// ..........
/// .S------7.
/// .|F----7|.
/// .||OOOO||.
/// .||OOOO||.
/// .|L-7F-J|.
/// .|II||II|.
/// .L--JL--J.
/// ..........
/// ```
///
/// In both of the above examples, `*4*` tiles are enclosed by the loop.
///
/// Here's a larger example:
///
/// ```ignore
/// .F----7F7F7F7F-7....
/// .|F--7||||||||FJ....
/// .||.FJ||||||||L7....
/// FJL7L7LJLJ||LJ.L-7..
/// L--J.L7...LJS7F-7L7.
/// ....F-J..F7FJ|L7L7L7
/// ....L7.F7||L7|.L7L7|
/// .....|FJLJ|FJ|F7|.LJ
/// ....FJL-7.||.||||...
/// ....L---J.LJ.LJLJ...
/// ```
///
/// The above sketch has many random bits of ground, some of which are in the
/// loop (`I`) and some of which are outside it (`O`):
///
/// ```ignore
/// OF----7F7F7F7F-7OOOO
/// O|F--7||||||||FJOOOO
/// O||OFJ||||||||L7OOOO
/// FJL7L7LJLJ||LJIL-7OO
/// L--JOL7IIILJS7F-7L7O
/// OOOOF-JIIF7FJ|L7L7L7
/// OOOOL7IF7||L7|IL7L7|
/// OOOOO|FJLJ|FJ|F7|OLJ
/// OOOOFJL-7O||O||||OOO
/// OOOOL---JOLJOLJLJOOO
/// ```
///
/// In this larger example, `*8*` tiles are enclosed by the loop.
///
/// Any tile that isn't part of the main loop can count as being enclosed by the
/// loop. Here's another example with many bits of junk pipe lying around that
/// aren't connected to the main loop at all:
///
/// ```ignore
/// FF7FSF7F7F7F7F7F---7
/// L|LJ||||||||||||F--J
/// FL-7LJLJ||||||LJL-77
/// F--JF--7||LJLJ7F7FJ-
/// L---JF-JLJ.||-FJLJJ7
/// |F|F-JF---7F7-L7L|7|
/// |FFJF7L7F-JF7|JL---7
/// 7-L-JL7||F7|L7F-7F7|
/// L.L7LFJ|||||FJL7||LJ
/// L7JLJL-JLJLJL--JLJ.L
/// ```
///
/// Here are just the tiles that are *enclosed by the loop* marked with `I`:
///
/// ```ignore
/// FF7FSF7F7F7F7F7F---7
/// L|LJ||||||||||||F--J
/// FL-7LJLJ||||||LJL-77
/// F--JF--7||LJLJIF7FJ-
/// L---JF-JLJIIIIFJLJJ7
/// |F|F-JF---7IIIL7L|7|
/// |FFJF7L7F-JF7IIL---7
/// 7-L-JL7||F7|L7F-7F7|
/// L.L7LFJ|||||FJL7||LJ
/// L7JLJL-JLJLJL--JLJ.L
/// ```
///
/// In this last example, `*10*` tiles are enclosed by the loop.
///
/// Figure out whether you have time to search for the nest by calculating the
/// area within the loop. *How many tiles are enclosed by the loop?*
pub fn solve_part_2(input: &str) -> Result<u32> {
    let lines = input.lines().collect();
    let mut maze = Maze::parse(lines)?;
    maze.walk_loop()?;
    maze.count_enclosed()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        // Load the file.
        let input = include_str!("../input/day10.txt");
        assert_eq!(super::solve_part_1(input).unwrap(), 7173);
    }

    #[test]
    fn test_solve_part_2() {
        // Load the file.
        let input = include_str!("../input/day10.txt");
        assert_eq!(super::solve_part_2(input).unwrap(), 291);
    }
}
