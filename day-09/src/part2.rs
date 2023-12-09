use itertools::Itertools;

use crate::custom_error::AocError;

pub fn extrapolate(sequence: &Vec<i64>) -> i64 {
    if sequence.iter().all(|&n| n == 0) {
        return 0;
    }
    let steps = sequence.windows(2).map(|w| w[1] - w[0]).collect();
    sequence.last().unwrap() + extrapolate(&steps)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut sum = 0;
    for line in input.lines() {
        let sequence = line
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect_vec()
            .into_iter()
            .rev()
            .collect_vec();
        sum += extrapolate(&sequence);
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
                     1 3 6 10 15 21
                     10 13 16 21 30 45";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
