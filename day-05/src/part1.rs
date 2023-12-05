use std::collections::{HashMap, VecDeque};

use crate::custom_error::AocError;

#[derive(Debug)]
pub struct MapRange {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

impl MapRange {
    fn map(&self, source: u64) -> Option<u64> {
        let src_range = self.source_start..self.source_start + self.length;
        if src_range.contains(&source) {
            let offset = source - self.source_start;
            return Some(self.destination_start + offset);
        }
        return None;
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

        return Self {
            source: source.to_string(),
            destination: destination.to_string(),
            ranges_map,
        };
    }

    fn map(&self, source: u64) -> u64 {
        for range in &self.ranges_map {
            if let Some(result) = range.map(source) {
                return result;
            }
        }
        return source;
    }
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

    let lowest_location = seeds
        .iter()
        .map(|seed| {
            let mut resource = "seed".to_string();
            let mut resource_id = seed.clone();
            while resource != "location" {
                let map = maps.get(&resource.to_string()).unwrap();
                resource = map.destination.clone();
                resource_id = map.map(resource_id);
            }
            resource_id
        })
        .min()
        .unwrap();

    Ok(lowest_location.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!("35", process(input)?);
        Ok(())
    }
}
