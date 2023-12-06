use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let lines = input.lines().collect::<Vec<_>>();
    let times = lines[0]
        .split_whitespace()
        .filter(|s| s != &"Time:")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let distances = lines[1]
        .split_whitespace()
        .filter(|s| s != &"Distance:")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // Merge times and distances into a vector of tuples
    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| (*t, *d))
        .collect::<Vec<_>>();

    let mut result = 1;
    for race in races {
        let (race_time, best_distance) = race;
        let mut better_results = 0;
        println!("Race: time={}, distance={}", race_time, best_distance);
        for time in 1..race_time - 1 {
            let speed = time;
            let distance = (race_time - time) * speed;
            if distance > best_distance {
                better_results += 1;
            }
        }
        result *= better_results;
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
        assert_eq!("288", process(input)?);
        Ok(())
    }
}
