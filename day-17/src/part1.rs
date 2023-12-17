use std::{collections::HashMap, i64::MAX};
use tracing::{debug, info, warn};

use crate::{
    custom_error::AocError,
    utils::{CharMap, Direction, Point},
};

fn find_min_loss(
    map: &CharMap,
    min_loss: &mut HashMap<Point<i64>, i64>,
    total_loss: i64,
    cur_pos: Point<i64>,
    cur_dir: Direction,
    steps_in_dir: usize,
) {
    debug!(
        "Visiting {:?} with loss {} and direction {:?}",
        cur_pos, total_loss, cur_dir
    );

    // Cannot take more than 3 steps in the same direction
    if steps_in_dir > 3 {
        debug!("- Cannot take more than 3 steps in the same direction");
        return;
    }

    // Cannot go out of bounds
    if map.out_of_bounds(&cur_pos) {
        debug!("- Cannot go out of bounds");
        return;
    }

    // Calculate the loss for the current path (including the current block)
    let current_block_loss = map.cell_digit_for_point(&cur_pos);
    let total_loss = total_loss + current_block_loss;

    // Check min loss for the current position
    let finish = map.bottom_right();
    if let Some(previous_min) = min_loss.get(&cur_pos) {
        if total_loss >= *previous_min {
            debug!("We have already been here, but with a lower loss");
            return;
        }

        if let Some(finish_loss) = min_loss.get(&finish) {
            if total_loss >= *finish_loss {
                debug!("We already have a better solution");
                return;
            }
        }
    }

    // Update min loss for the current position
    min_loss.insert(cur_pos, total_loss);

    // Check if we reached the end
    if cur_pos == finish {
        info!("+ Reached the end with loss {}", total_loss);
        return;
    }

    // Now try going forward, left and right
    debug!("+ Going forward");
    let next_pos = cur_pos + cur_dir.delta();
    let steps_in_dir = steps_in_dir + 1;
    find_min_loss(map, min_loss, total_loss, next_pos, cur_dir, steps_in_dir);

    debug!("+ Going left");
    let next_dir = cur_dir.turn_left();
    let next_pos = cur_pos + next_dir.delta();
    find_min_loss(map, min_loss, total_loss, next_pos, next_dir, 1);

    debug!("+ Going right");
    let next_dir = cur_dir.turn_right();
    let next_pos = cur_pos + next_dir.delta();
    find_min_loss(map, min_loss, total_loss, next_pos, next_dir, 1);
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map = CharMap::from_str_with_trim(input, '#');

    let mut min_loss = HashMap::new();
    let start = map.top_left();

    // Start by going east
    min_loss.insert(start, MAX);
    find_min_loss(&map, &mut min_loss, 0, start, Direction::East, 0);

    // Then go south
    min_loss.insert(start, MAX);
    find_min_loss(&map, &mut min_loss, 0, start, Direction::South, 0);

    // Since we should not count the loss from the first block, we need to subtract it
    let starting_loss = map.cell_digit_for_point(&start);
    let finish = map.bottom_right();
    let finishing_loss = min_loss[&finish];
    let result = finishing_loss - starting_loss;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2413432311323
                     3215453535623
                     3255245654254
                     3446585845452
                     4546657867536
                     1438598798454
                     4457876987766
                     3637877979653
                     4654967986887
                     4564679986453
                     1224686865563
                     2546548887735
                     4322674655533";
        assert_eq!("102", process(input)?);
        Ok(())
    }
}

// Submissions:
// 2203 - too high
// 1183 - too high
// ...
// 1132 - too low
