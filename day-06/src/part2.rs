use std::cmp::min;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let lines = input.lines().collect::<Vec<_>>();
    let times = lines[0]
        .split_whitespace()
        .filter(|s| s != &"Time:")
        .collect::<Vec<_>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    let distances = lines[1]
        .split_whitespace()
        .filter(|s| s != &"Distance:")
        .collect::<Vec<_>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    // Merge times and distances into a vector of tuples
    let race = (times, distances);
    let (race_time, best_distance) = race;
    println!("Race: time={}, distance={}", race_time, best_distance);

    // (race_time - wait_time) * wait_time = best_distance
    // So we should only start looking for better results after wait_time
    let d = ((race_time * race_time - 4 * best_distance) as f64).sqrt();
    let wait_time = min(
        -(0 as i64 - race_time + d.round() as i64) / 2,
        -(0 as i64 - race_time - d.round() as i64) / 2,
    );
    println!("Best wait time: {}", wait_time);

    let mut better_results_start = 0;
    for time in wait_time..race_time - 1 {
        let speed = time;
        let distance = (race_time - time) * speed;
        if distance > best_distance {
            better_results_start = time;
            break;
        }
    }

    println!("Better results start at {}", better_results_start);
    let result = race_time - better_results_start * 2 + 1;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
                     Distance:  9  40  200";
        assert_eq!("71503", process(input)?);
        Ok(())
    }
}
