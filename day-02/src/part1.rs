use std::collections::HashMap;

use crate::custom_error::AocError;

pub fn possible_draw(draw: &str, limits: &HashMap<&str, u32>) -> bool {
    let draw_cubes = draw.split(",").collect::<Vec<&str>>();
    for cube in draw_cubes {
        let cube_parts = cube.trim().split(" ").collect::<Vec<&str>>();
        let color = cube_parts[1];
        let count = cube_parts[0].parse::<u32>().unwrap();

        if count > *limits.get(color).unwrap() {
            return false;
        }
    }
    return true;
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut limits: HashMap<&str, u32> = HashMap::new();
    limits.insert("red", 12);
    limits.insert("green", 13);
    limits.insert("blue", 14);

    let mut possible_games_sum = 0;
    for line in input.lines() {
        let game_parts = line.split(":").collect::<Vec<&str>>();
        let game_number = game_parts[0].trim().split(" ").collect::<Vec<&str>>()[1];
        let game = game_parts[1].trim();
        let draws = game.split(";").collect::<Vec<&str>>();

        let mut game_possible = true;
        for draw in draws {
            if !possible_draw(draw, &limits) {
                game_possible = false;
                break;
            }
        }

        if game_possible {
            possible_games_sum += game_number.parse::<u32>().unwrap();
        }
    }

    return Ok(possible_games_sum.to_string());
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
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
