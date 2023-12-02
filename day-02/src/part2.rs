use crate::custom_error::AocError;
use std::collections::HashMap;

pub fn update_limits<'a>(draw: &'a str, limits: &mut HashMap<&'a str, u32>) {
    let draw_cubes = draw.split(",").collect::<Vec<&str>>();
    for cube in draw_cubes {
        let cube_parts = cube.trim().split(" ").collect::<Vec<&str>>();
        let color = cube_parts[1];
        let count = cube_parts[0].parse::<u32>().unwrap();
        if count > *limits.get(color).unwrap() {
            limits.insert(color, count);
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut games_sum = 0;

    for line in input.lines() {
        let game = line.split(":").last().unwrap().trim();
        let draws = game.split(";").collect::<Vec<&str>>();

        let mut limits = HashMap::new();
        limits.insert("red", 0);
        limits.insert("green", 0);
        limits.insert("blue", 0);

        for draw in draws {
            update_limits(draw, &mut limits);
        }

        let set_power: u32 = limits.values().product();
        games_sum += set_power;
    }

    return Ok(games_sum.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
