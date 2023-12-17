use fxhash::FxHashMap;

use crate::{
    custom_error::AocError,
    utils::Direction,
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
    min_loss: &mut FxHashMap<Step, i64>,
    best_total_loss: &mut i64,
    total_loss: i64,
    current: Step,
) {
    // Cannot take more than 10 steps in the same direction
    if current.steps > 10 {
        return;
    }

    // Cannot go out of bounds
    if map.out_of_bounds(&current.pos) {
        return;
    }

    // Calculate the loss for the current path (including the current block)
    let current_block_loss = map.cell_digit_for_point(&current.pos);
    let total_loss = total_loss + current_block_loss;

    // We can only consider a step after we have taken 4 steps in the same direction
    if current.steps >= 4 {
        // Check against the current known best total loss
        if total_loss >= *best_total_loss {
            return;
        }

        // Check min loss for the current position
        if let Some(previous_min) = min_loss.get(&current) {
            if total_loss >= *previous_min {
                return;
            }
        }
        min_loss.insert(current, total_loss);

        // Check if we reached the end
        if current.pos == map.bottom_right() {
            if total_loss < *best_total_loss {
                println!("New best loss: {}", total_loss);
                *best_total_loss = total_loss;
            }
            return;
        }
    }

    // Now try going forward, left and right
    let forward = Step {
        dir: current.dir,
        pos: current.pos + current.dir.delta(),
        steps: current.steps + 1,
    };
    find_min_loss(map, min_loss, best_total_loss, total_loss, forward);

    if current.steps >= 4 {
        let left = Step {
            dir: current.dir.turn_left(),
            pos: current.pos + current.dir.turn_left().delta(),
            steps: 1,
        };
        find_min_loss(map, min_loss, best_total_loss, total_loss, left);

        let right = Step {
            dir: current.dir.turn_right(),
            pos: current.pos + current.dir.turn_right().delta(),
            steps: 1,
        };
        find_min_loss(map, min_loss, best_total_loss, total_loss, right);
    }
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let map = CharMap::from_str_with_trim(input, '#');

    let mut best_total_loss = 2000; //std::i64::MAX;
    let mut min_loss_per_step = FxHashMap::default();

    // Go east
    let east = Step {
        pos: Point::new(1, 0),
        dir: Direction::East,
        steps: 1,
    };
    find_min_loss(&map, &mut min_loss_per_step, &mut best_total_loss, 0, east);

    // Go south
    let south = Step {
        pos: Point::new(0, 1),
        dir: Direction::South,
        steps: 1,
    };
    find_min_loss(&map, &mut min_loss_per_step, &mut best_total_loss, 0, south);

    Ok(best_total_loss.to_string())
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
        assert_eq!("94", process(input)?);
        Ok(())
    }

    #[test]
    fn test_another_process() -> miette::Result<()> {
        tracing_subscriber::fmt::init();

        let input = "111111111111
                     999999999991
                     999999999991
                     999999999991
                     999999999991";
        assert_eq!("71", process(input)?);
        Ok(())
    }
}

// Submissions: 1283 - correct
