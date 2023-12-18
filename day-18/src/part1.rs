use itertools::Itertools;

use crate::{
    custom_error::AocError,
    utils::{CharMap, Point},
};

#[derive(Debug)]
struct Dig {
    map: CharMap,
    pos: Point<i64>,
    max_x: i64,
    max_y: i64,
}

impl Dig {
    fn new() -> Self {
        let map_size = 10000;
        let mut dig = Self {
            map: CharMap::from_dimensions(map_size, map_size, '.'),
            pos: Point::new(
                (map_size / 2).try_into().unwrap(),
                (map_size / 2).try_into().unwrap(),
            ),
            max_x: 0,
            max_y: 0,
        };
        dig.map
            .set_cell(dig.pos.x as usize, dig.pos.y as usize, '#');
        dig
    }

    fn dig(&mut self, dir: &str, steps: i64, _color: &str) {
        for _ in 0..steps {
            match dir {
                "R" => self.pos.x += 1,
                "L" => self.pos.x -= 1,
                "U" => self.pos.y -= 1,
                "D" => self.pos.y += 1,
                _ => panic!("Unknown direction {}", dir),
            }

            if self.pos.x < 0 || self.pos.y < 0 {
                panic!("Out of bounds");
            }

            if self.pos.x > self.max_x {
                self.max_x = self.pos.x;
            }

            if self.pos.y > self.max_y {
                self.max_y = self.pos.y;
            }

            self.map
                .set_cell(self.pos.x as usize, self.pos.y as usize, '#');
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut dig = Dig::new();
    for line in input.lines() {
        let line = line.trim();
        let parts = line.split(' ').collect_vec();
        let dir = parts[0];
        let steps = parts[1].parse::<i64>().unwrap();
        let color = parts[2].trim_start_matches('(').trim_end_matches(')');

        println!("{} {} {}", dir, steps, color);
        dig.dig(dir, steps, color);
    }

    println!("Max x: {}", dig.max_x);
    println!("Max y: {}", dig.max_y);

    // dig.map.print();
    let mut count = dig.map.count('#');
    dig.map.flood_fill(dig.map.top_left(), 'O');
    count = count + dig.map.count('.');

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "R 6 (#70c710)
                     D 5 (#0dc571)
                     L 2 (#5713f0)
                     D 2 (#d2c081)
                     R 2 (#59c680)
                     D 2 (#411b91)
                     L 5 (#8ceee2)
                     U 2 (#caa173)
                     L 1 (#1b58a2)
                     U 2 (#caa171)
                     R 2 (#7807d2)
                     U 3 (#a77fa3)
                     L 2 (#015232)
                     U 2 (#7a21e3)";
        assert_eq!("62", process(input)?);
        Ok(())
    }
}
