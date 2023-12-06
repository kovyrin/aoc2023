use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let lines = input.lines().collect::<Vec<_>>();
    let times = lines[0]
        .split_whitespace()
        .filter(|s| s != &"Time:")
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let distances = lines[1]
        .split_whitespace()
        .filter(|s| s != &"Distance:")
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    // Merge times and distances into a vector of tuples
    let race = (times, distances);
    let (race_time, best_distance) = race;
    let mut result = 0;
    println!("Race: time={}, distance={}", race_time, best_distance);

    for time in 1..race_time - 1 {
        let speed = time;
        let distance = (race_time - time) * speed;
        if distance > best_distance {
            result += 1;
        }
    }

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
