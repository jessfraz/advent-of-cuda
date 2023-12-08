//!  Day 05: If You Give A Seed A Fertilizer
use std::sync::Arc;

use anyhow::Result;
use rayon::prelude::*;

/// Data for the almanac.
#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<(u64, u64, u64)>,
    soil_to_fertilizer: Vec<(u64, u64, u64)>,
    fertilizer_to_water: Vec<(u64, u64, u64)>,
    water_to_light: Vec<(u64, u64, u64)>,
    light_to_temperature: Vec<(u64, u64, u64)>,
    temperature_to_humidity: Vec<(u64, u64, u64)>,
    humidity_to_location: Vec<(u64, u64, u64)>,
}

/// The data for a seed.
#[derive(Debug)]
struct Seed {
    location: u64,
}

impl Seed {
    /// Create a new seed from a seed value.
    fn from_u64(almanac: &Arc<Almanac>, seed: u64) -> Result<Self> {
        let soil = almanac.solve_seed_to_soil(seed)?;
        let fertilizer = almanac.solve_soil_to_fertilizer(soil)?;
        let water = almanac.solve_fertilizer_to_water(fertilizer)?;
        let light = almanac.solve_water_to_light(water)?;
        let temperature = almanac.solve_light_to_temperature(light)?;
        let humidity = almanac.solve_temperature_to_humidity(temperature)?;
        let location = almanac.solve_humidity_to_location(humidity)?;

        Ok(Seed { location })
    }
}

impl Almanac {
    /// Solve the almanac for each seed.
    fn solve(&self) -> Result<Vec<Seed>> {
        let almanac = Arc::new(self.clone());
        let seeds = self
            .seeds
            .par_iter()
            .map(|seed| {
                Seed::from_u64(&almanac, *seed).expect("Failed to solve seed")
            })
            .collect::<Vec<Seed>>();
        Ok(seeds)
    }

    /// Expand the seed ranges for the almanac.
    fn expand_seed_ranges(&mut self) -> Result<()> {
        let mut new_seeds = Vec::new();
        let mut old_seeds = self.seeds.clone().into_iter();

        while let Some(seed_start) = old_seeds.next() {
            let seed_len = old_seeds.next().ok_or_else(|| {
                anyhow::anyhow!("No length for seed starting at {}", seed_start)
            })?;

            for seed in seed_start..seed_start + seed_len {
                new_seeds.push(seed);
            }
        }

        self.seeds = new_seeds;

        Ok(())
    }

    /// Solve the seed to soil.
    fn solve_seed_to_soil(&self, seed: u64) -> Result<u64> {
        for (dest_start, src_start, len) in &self.seed_to_soil {
            if seed >= *src_start && seed < *src_start + *len {
                return Ok(seed - src_start + dest_start);
            }
        }

        Ok(seed)
    }

    /// Solve the soil to fertilizer.
    fn solve_soil_to_fertilizer(&self, soil: u64) -> Result<u64> {
        for (dest_start, src_start, len) in &self.soil_to_fertilizer {
            if soil >= *src_start && soil < *src_start + *len {
                return Ok(soil - src_start + dest_start);
            }
        }

        Ok(soil)
    }

    /// Solve the fertilizer to water.
    fn solve_fertilizer_to_water(&self, fertilizer: u64) -> Result<u64> {
        for (dest_start, src_start, len) in &self.fertilizer_to_water {
            if fertilizer >= *src_start && fertilizer < *src_start + *len {
                return Ok(fertilizer - src_start + dest_start);
            }
        }

        Ok(fertilizer)
    }

    /// Solve the water to light.
    fn solve_water_to_light(&self, water: u64) -> Result<u64> {
        for (dest_start, src_start, len) in &self.water_to_light {
            if water >= *src_start && water < *src_start + *len {
                return Ok(water - src_start + dest_start);
            }
        }

        Ok(water)
    }

    /// Solve the light to temperature.
    fn solve_light_to_temperature(&self, light: u64) -> Result<u64> {
        for (dest_start, src_start, len) in &self.light_to_temperature {
            if light >= *src_start && light < *src_start + *len {
                return Ok(light - src_start + dest_start);
            }
        }

        Ok(light)
    }

    /// Solve the temperature to humidity.
    fn solve_temperature_to_humidity(&self, temperature: u64) -> Result<u64> {
        for (dest_start, src_start, len) in &self.temperature_to_humidity {
            if temperature >= *src_start && temperature < *src_start + *len {
                return Ok(temperature - src_start + dest_start);
            }
        }

        Ok(temperature)
    }

    /// Solve the humidity to location.
    fn solve_humidity_to_location(&self, humidity: u64) -> Result<u64> {
        for (dest_start, src_start, len) in &self.humidity_to_location {
            if humidity >= *src_start && humidity < *src_start + *len {
                return Ok(humidity - src_start + dest_start);
            }
        }

        Ok(humidity)
    }
}

/// Get the data of the almanac from the input string.
fn parse_almanac(input: &str) -> Result<Almanac> {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("No seeds in the almanac"))?
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;
    lines.next(); // skip empty line
    let seed_to_soil = parse_map(&mut lines, "seed-to-soil")?;
    let soil_to_fertilizer = parse_map(&mut lines, "soil-to-fertilizer")?;
    let fertilizer_to_water = parse_map(&mut lines, "fertilizer-to-water")?;
    let water_to_light = parse_map(&mut lines, "water-to-light")?;
    let light_to_temperature = parse_map(&mut lines, "light-to-temperature")?;
    let temperature_to_humidity =
        parse_map(&mut lines, "temperature-to-humidity")?;
    let humidity_to_location = parse_map(&mut lines, "humidity-to-location")?;
    Ok(Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    })
}

/// Parse a map from the input lines.
fn parse_map<'a, I>(lines: &mut I, name: &str) -> Result<Vec<(u64, u64, u64)>>
where
    I: Iterator<Item = &'a str>,
{
    let mut map = Vec::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        if line.starts_with(name) {
            continue;
        }
        let mut parts = line.split_whitespace();
        let dest_start = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing destination start"))?
            .parse::<u64>()?;
        let src_start = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing source start"))?
            .parse::<u64>()?;
        let len = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing length"))?
            .parse::<u64>()?;
        map.push((dest_start, src_start, len));
    }
    Ok(map)
}

/// You take the boat and find the gardener right where you were told he would
/// be: managing a giant "garden" that looks more to you like a farm.
///
/// "A water source? Island Island *is* the water source!" You point out that
/// Snow Island isn't receiving any water.
///
/// "Oh, we had to stop the water because we *ran out of sand* to filter (<https://en.wikipedia.org/wiki/Sand_filter>) it with! Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few days... weeks... oh no." His face sinks into a look of horrified realization.
///
/// "I've been so busy making sure everyone here has food that I completely
/// forgot to check why we stopped getting more sand! There's a ferry leaving
/// soon that is headed over in that direction - it's much faster than your
/// boat. Could you please go check it out?"
///
/// You barely have time to agree to this request when he brings up another. "While you wait for the ferry, maybe you can help us with our *food production problem*. The latest Island Island Almanac (<https://en.wikipedia.org/wiki/Almanac>) just arrived and we're having trouble making sense of it."
///
/// The almanac (your puzzle input) lists all of the seeds that need to be
/// planted. It also lists what type of soil to use with each kind of seed, what
/// type of fertilizer to use with each kind of soil, what type of water to use
/// with each kind of fertilizer, and so on. Every type of seed, soil,
/// fertilizer and so on is identified with a number, but numbers are reused by
/// each category - that is, soil `123` and fertilizer `123` aren't necessarily
/// related to each other.
///
/// For example:
///
/// ```ignore
/// seeds: 79 14 55 13
///
/// seed-to-soil map:
/// 50 98 2
/// 52 50 48
///
/// soil-to-fertilizer map:
/// 0 15 37
/// 37 52 2
/// 39 0 15
///
/// fertilizer-to-water map:
/// 49 53 8
/// 0 11 42
/// 42 0 7
/// 57 7 4
///
/// water-to-light map:
/// 88 18 7
/// 18 25 70
///
/// light-to-temperature map:
/// 45 77 23
/// 81 45 19
/// 68 64 13
///
/// temperature-to-humidity map:
/// 0 69 1
/// 1 0 69
///
/// humidity-to-location map:
/// 60 56 37
/// 56 93 4
/// ```
///
/// The almanac starts by listing which seeds need to be planted: seeds `79`,
/// `14`, `55`, and `13`.
///
/// The rest of the almanac contains a list of *maps* which describe how to
/// convert numbers from a *source category* into numbers in a *destination
/// category*. That is, the section that starts with `seed-to-soil map:`
/// describes how to convert a *seed number* (the source) to a *soil number*
/// (the destination). This lets the gardener and his team know which soil to
/// use with which seeds, which water to use with which fertilizer, and so on.
///
/// Rather than list every source number and its corresponding destination
/// number one by one, the maps describe entire *ranges* of numbers that can be
/// converted. Each line within a map contains three numbers: the *destination
/// range start*, the *source range start*, and the *range length*.
///
/// Consider again the example `seed-to-soil map`:
///
/// ```ignore
/// 50 98 2
/// 52 50 48
/// ```
///
/// The first line has a *destination range start* of `50`, a *source range
/// start* of `98`, and a *range length* of `2`. This line means that the source
/// range starts at `98` and contains two values: `98` and `99`. The destination
/// range is the same length, but it starts at `50`, so its two values are `50`
/// and `51`. With this information, you know that seed number `98` corresponds
/// to soil number `50` and that seed number `99` corresponds to soil number
/// `51`.
///
/// The second line means that the source range starts at `50` and contains `48`
/// values: `50`, `51`, ..., `96`, `97`. This corresponds to a destination range
/// starting at `52` and also containing `48` values: `52`, `53`, ..., `98`,
/// `99`. So, seed number `53` corresponds to soil number `55`.
///
/// Any source numbers that *aren't mapped* correspond to the *same* destination
/// number. So, seed number `10` corresponds to soil number `10`.
///
/// So, the entire list of seed numbers and their corresponding soil numbers
/// looks like this:
///
/// ```ignore
/// seed  soil
/// 0     0
/// 1     1
/// ...   ...
/// 48    48
/// 49    49
/// 50    52
/// 51    53
/// ...   ...
/// 96    98
/// 97    99
/// 98    50
/// 99    51
/// ```
///
/// With this map, you can look up the soil number required for each initial
/// seed number:
///
/// * Seed number `79` corresponds to soil number `81`.
/// * Seed number `14` corresponds to soil number `14`.
/// * Seed number `55` corresponds to soil number `57`.
/// * Seed number `13` corresponds to soil number `13`.
///
/// The gardener and his team want to get started as soon as possible, so they'd
/// like to know the closest location that needs a seed. Using these maps, find
/// *the lowest location number that corresponds to any of the initial seeds*.
/// To do this, you'll need to convert each seed number through other categories
/// until you can find its corresponding *location number*. In this example, the
/// corresponding types are:
///
/// * Seed `79`, soil `81`, fertilizer `81`, water `81`, light `74`, temperature
///   `78`, humidity `78`, *location `82`*.
/// * Seed `14`, soil `14`, fertilizer `53`, water `49`, light `42`, temperature
///   `42`, humidity `43`, *location `43`*.
/// * Seed `55`, soil `57`, fertilizer `57`, water `53`, light `46`, temperature
///   `82`, humidity `82`, *location `86`*.
/// * Seed `13`, soil `13`, fertilizer `52`, water `41`, light `34`, temperature
///   `34`, humidity `35`, *location `35`*.
///
/// So, the lowest location number in this example is `*35*`.
///
/// *What is the lowest location number that corresponds to any of the initial
/// seed numbers?*
pub fn solve_part_1(input: &str) -> Result<u64> {
    let almanac = parse_almanac(input)?;

    let seeds = almanac.solve()?;

    // Find the lowest location number that corresponds to any of the initial
    // seed numbers.
    let lowest_location = seeds
        .par_iter()
        .map(|s| s.location)
        .min()
        .ok_or_else(|| anyhow::anyhow!("No seeds found in the almanac"))?;

    Ok(lowest_location)
}

/// Everyone will starve if you only plant such a small number of seeds.
/// Re-reading the almanac, it looks like the `seeds:` line actually describes
/// *ranges of seed numbers*.
///
/// The values on the initial `seeds:` line come in pairs. Within each pair, the
/// first value is the *start* of the range and the second value is the *length*
/// of the range. So, in the first line of the example above:
///
/// ```ignore
/// seeds: 79 14 55 13
/// ```
///
/// This line describes two ranges of seed numbers to be planted in the garden.
/// The first range starts with seed number `79` and contains `14` values: `79`,
/// `80`, ..., `91`, `92`. The second range starts with seed number `55` and
/// contains `13` values: `55`, `56`, ..., `66`, `67`.
///
/// Now, rather than considering four seed numbers, you need to consider a total
/// of *27* seed numbers.
///
/// In the above example, the lowest location number can be obtained from seed
/// number `82`, which corresponds to soil `84`, fertilizer `84`, water `84`,
/// light `77`, temperature `45`, humidity `46`, and *location `46`*. So, the
/// lowest location number is `*46*`.
///
/// Consider all of the initial seed numbers listed in the ranges on the first
/// line of the almanac. *What is the lowest location number that corresponds to
/// any of the initial seed numbers?*
pub fn solve_part_2(input: &str) -> Result<u64> {
    let mut almanac = parse_almanac(input)?;

    // Expand the seed ranges.
    almanac.expand_seed_ranges()?;

    let seeds = almanac.solve()?;

    // Save memory.
    drop(almanac);

    // Find the lowest location number that corresponds to any of the initial
    // seed numbers.
    let lowest_location = seeds
        .par_iter()
        .map(|s| s.location)
        .min()
        .ok_or_else(|| anyhow::anyhow!("No seeds found in the almanac"))?;

    Ok(lowest_location)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        // Load the file.
        let input = include_str!("../input/day05.txt");
        assert_eq!(super::solve_part_1(input).unwrap(), 214922730);
    }

    #[test]
    fn test_solve_part_2() {
        // Load the file.
        let input = include_str!("../input/day05.txt");
        assert_eq!(super::solve_part_2(input).unwrap(), 148041808);
    }
}
