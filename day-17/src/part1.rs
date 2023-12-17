use std::collections::HashMap;
use tracing::{debug, info, warn};

use crate::{
    custom_error::AocError,
    utils::Direction::{self, *},
    utils::{CharMap, Point},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Step {
    pos: Point<i64>,
    dir: Direction,
    steps: usize,
}

fn find_min_loss(
    map: &CharMap,
    min_loss: &mut HashMap<Step, i64>,
    best_total_loss: &mut i64,
    total_loss: i64,
    current: Step,
) {
    debug!(
        "Visiting {:?} with loss {} and direction {:?}",
        current.pos, total_loss, current.dir
    );

    // Cannot take more than 3 steps in the same direction
    if current.steps > 3 {
        debug!("- Cannot take more than 3 steps in the same direction");
        return;
    }

    // Cannot go out of bounds
    if map.out_of_bounds(&current.pos) {
        debug!("- Cannot go out of bounds");
        return;
    }

    // Calculate the loss for the current path (including the current block)
    let current_block_loss = map.cell_digit_for_point(&current.pos);
    let total_loss = total_loss + current_block_loss;
    debug!(
        "* Current block loss is {}, total loss is {}",
        current_block_loss, total_loss
    );

    // Check against the current known best total loss
    if total_loss >= *best_total_loss {
        debug!("- Worse than the best solution: {}", best_total_loss);
        return;
    }

    // Check min loss for the current position
    if let Some(previous_min) = min_loss.get(&current) {
        debug!("* Best loss for this point: {}", previous_min);
        if total_loss >= *previous_min {
            debug!("- We have already been here, but with the same or lower loss");
            return;
        } else {
            debug!("+ This is better for {:?}!", current.pos);
        }
    } else {
        debug!("* First time here");
    }

    min_loss.insert(current, total_loss);

    // Check if we reached the end
    if current.pos == map.bottom_right() {
        if total_loss < *best_total_loss {
            info!("+ Reached the end with a new best: {}", total_loss);
            *best_total_loss = total_loss;
        }
        return;
    }

    // Now try going forward, left and right
    debug!("+ Going forward from {:?}", current.pos);
    let next = Step {
        dir: current.dir,
        pos: current.pos + current.dir.delta(),
        steps: current.steps + 1,
    };
    find_min_loss(map, min_loss, best_total_loss, total_loss, next);

    debug!("+ Going left from {:?}", current.pos);
    let next = Step {
        dir: current.dir.turn_left(),
        pos: current.pos + current.dir.turn_left().delta(),
        steps: 1,
    };
    find_min_loss(map, min_loss, best_total_loss, total_loss, next);

    debug!("+ Going right from {:?}", current.pos);
    let next = Step {
        dir: current.dir.turn_right(),
        pos: current.pos + current.dir.turn_right().delta(),
        steps: 1,
    };
    find_min_loss(map, min_loss, best_total_loss, total_loss, next);
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map = CharMap::from_str_with_trim(input, '#');

    let mut best_total_loss = std::i64::MAX;
    let mut min_loss_per_step = HashMap::new();

    // Go east
    find_min_loss(
        &map,
        &mut min_loss_per_step,
        &mut best_total_loss,
        0,
        Step {
            pos: Point::new(1, 0),
            dir: East,
            steps: 1,
        },
    );
    // debug_min_loss(&map, &min_loss, &best_path);

    // Go south
    find_min_loss(
        &map,
        &mut min_loss_per_step,
        &mut best_total_loss,
        0,
        Step {
            pos: Point::new(0, 1),
            dir: Direction::South,
            steps: 1,
        },
    );
    // debug_min_loss(&map, &min_loss, &best_path);

    Ok(best_total_loss.to_string())
}

fn debug_min_loss(map: &CharMap, min_loss: &HashMap<Point<i64>, i64>, best_path: &Vec<Step>) {
    println!("Map:");
    map.print();

    println!("Path:");
    println!("{:?}", best_path);
    for row in 0..map.height() {
        for col in 0..map.width() {
            let pos = Point::new(col as i64, row as i64);
            let path_step = best_path.iter().find(|s| s.pos == pos);
            if let Some(step) = path_step {
                print!("{}", step.dir.to_char());
            } else {
                print!("{}", map.cell_for_point(&pos));
            }
        }
        println!();
    }

    println!("Min loss map:");
    for row in 0..map.height() {
        for col in 0..map.width() {
            let pos = Point::new(col as i64, row as i64);
            print!("{:4} ", min_loss.get(&pos).unwrap_or(&0));
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        tracing_subscriber::fmt::init();

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
// 1185 - too high
// 1183 - too high
// 1155 - correct!
// 1132 - too low
