use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
    u64::MAX,
};

use crate::custom_error::AocError;

#[derive(Debug)]
pub struct MapRange {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

impl MapRange {
    // Receives a source range and returns a destination range
    fn map(&self, source_start: u64, source_end: u64) -> Option<(Range<u64>, u64)> {
        if !self.source_range().contains(&source_start) {
            return None;
        }

        //........[--source---]..
        //.....[------]..........
        println!("  found range {:?}", self);
        let match_start = source_start;
        let match_end = std::cmp::min(source_end, self.source_end());
        let match_offset = source_start - self.source_start;

        let dest_start = self.destination_start + match_offset;
        let dest_end = dest_start + (match_end - match_start);
        return Some((dest_start..dest_end, match_end));
    }

    fn source_range(&self) -> Range<u64> {
        self.source_start..self.source_start + self.length
    }

    fn source_end(&self) -> u64 {
        self.source_start + self.length
    }
}

#[derive(Debug)]
pub struct SomethingToSomethingMap {
    source: String,
    destination: String,
    ranges_map: Vec<MapRange>,
}

impl SomethingToSomethingMap {
    fn from_lines(lines: &mut VecDeque<&str>) -> Self {
        let header = lines.pop_front().unwrap();
        let src_to_dest = header.split_whitespace().nth(0).unwrap();
        let parts = src_to_dest.split("-to-");
        let source = parts.clone().nth(0).unwrap();
        let destination = parts.clone().nth(1).unwrap();

        let mut ranges_map = Vec::new();
        loop {
            match lines.pop_front() {
                Some("") => break,
                Some(line) => {
                    let mut parts = line.split_whitespace();
                    let destination_start = parts.next().unwrap().parse::<u64>().unwrap();
                    let source_start = parts.next().unwrap().parse::<u64>().unwrap();
                    let length = parts.next().unwrap().parse::<u64>().unwrap();
                    ranges_map.push(MapRange {
                        source_start,
                        destination_start,
                        length,
                    });
                }
                _ => break,
            }
        }

        ranges_map.sort_by_key(|r| r.source_start);

        return Self {
            source: source.to_string(),
            destination: destination.to_string(),
            ranges_map,
        };
    }

    // Returns the destination range and the new start for the source range
    fn map(&self, source_start: u64, source_end: u64) -> (Range<u64>, u64) {
        for range in &self.ranges_map {
            println!("  checking range {:?} for {}", range, source_start);
            if let Some((dest_range, match_end)) = range.map(source_start, source_end) {
                return (dest_range, match_end);
            }
        }

        let applicable_ranges = self
            .ranges_map
            .iter()
            .filter(|r| r.source_end() > source_start);

        if applicable_ranges.clone().count() == 0 {
            println!(
                "  no applicable ranges, returning {}..{}",
                source_start, source_end
            );
            return (source_start..source_end, source_end);
        }

        let min_range_start = applicable_ranges.clone().map(|r| r.source_start).min();
        let max_range_end = applicable_ranges.clone().map(|r| r.source_end()).max();

        println!("  source_start: {}", source_start);
        println!("  source_end: {}", source_end);
        println!("  min_range_start: {:?}", min_range_start);
        println!("  max_range_end: {:?}", max_range_end);

        //.............[---source---].
        //..[--] [--].................
        if max_range_end.is_some() && source_start >= max_range_end.unwrap() {
            println!(
                "  source range after all ranges, returning {}..{}",
                source_start, source_end
            );
            return (source_start..source_end, source_end);
        }

        if min_range_start.is_some() {
            let min_range_start = min_range_start.unwrap();

            //.[---source---].............
            //.................[--] [--]..
            if source_end <= min_range_start {
                println!(
                    "  source range before all ranges, returning {}..{}",
                    source_start, source_end
                );
                return (source_start..source_end, source_end);
            }

            //.[---source---].........
            //...........[----] [--]..
            if source_start < min_range_start && source_end >= min_range_start {
                println!(
                    "  source range overlaps start of ranges, returning prefix only {}..{}",
                    source_start, min_range_start
                );
                return (source_start..min_range_start, min_range_start);
            }
        }

        panic!("unhandled case");
    }

    fn map_to_ranges(&self, source_start: u64, source_end: u64) -> Vec<Range<u64>> {
        let mut cur_start = source_start;
        let cur_end = source_end;
        let mut dest_ranges = Vec::new();

        while cur_start < cur_end {
            println!("Range left to map: {}..{}", cur_start, cur_end);
            let (dest_range, match_end) = self.map(cur_start, cur_end);
            cur_start = match_end;
            println!("  pushing range {:?}", dest_range);
            dest_ranges.push(dest_range);
        }

        return dest_ranges;
    }
}

fn min_location_for_range(
    resource: String,
    source_start: u64,
    source_end: u64,
    maps: &HashMap<String, SomethingToSomethingMap>,
) -> u64 {
    println!(
        "min_location_for_range({}, {}, {})",
        resource, source_start, source_end
    );

    // Find the map of this resource to the next one
    let map = maps.get(&resource).unwrap();

    // Map the source range to a list of destination ranges
    let dest_ranges = map.map_to_ranges(source_start, source_end);

    // If this map maps into locations, find the minimum location
    let dest_resource = &map.destination;
    if dest_resource == "location" {
        return dest_ranges.iter().map(|r| r.start).min().unwrap();
    }

    // Otherwise, recurse into the destination resource
    dest_ranges
        .iter()
        .map(|r| min_location_for_range(dest_resource.clone(), r.start, r.end, maps))
        .min()
        .unwrap()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut lines = input.lines().collect::<VecDeque<_>>();
    let seeds = lines
        .pop_front()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    lines.pop_front();

    let mut maps = HashMap::new();
    while !lines.is_empty() {
        let map = SomethingToSomethingMap::from_lines(&mut lines);
        maps.insert(map.source.clone(), map);
    }

    let mut lowest_location = MAX;
    for seed_range in seeds.chunks(2) {
        let range_start = seed_range[0];
        let range_len = seed_range[1];
        let range_end = range_start + range_len;
        dbg!(range_start, range_end);

        let min_location =
            min_location_for_range("seed".to_string(), range_start, range_end, &maps);

        if min_location < lowest_location {
            lowest_location = min_location;
        }
    }

    Ok(lowest_location.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_to_ranges() -> miette::Result<()> {
        let input = "seed-to-soil map:
        110 10 5
        220 20 20
        ";

        let mut lines = &mut input.lines().map(|s| s.trim()).collect::<VecDeque<_>>();
        let map = SomethingToSomethingMap::from_lines(&mut lines);
        let ranges = map.map_to_ranges(1, 110);
        assert_eq!(ranges.len(), 5);
        assert_eq!(ranges[0], 1..10);
        assert_eq!(ranges[1], 110..115);
        assert_eq!(ranges[2], 15..20);
        assert_eq!(ranges[3], 220..240);
        assert_eq!(ranges[4], 40..110);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4";
        assert_eq!("46", process(input)?);
        Ok(())
    }
}
