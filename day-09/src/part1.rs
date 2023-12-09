use itertools::Itertools;

use crate::custom_error::AocError;

pub fn extrapolate(sequence: &Vec<i64>) -> i64 {
    let mut steps = Vec::with_capacity(sequence.len() - 1);
    let mut has_non_zero = false;
    for i in 1..sequence.len() {
        let step = sequence[i] - sequence[i - 1];
        has_non_zero = has_non_zero || (step != 0);
        steps.push(step);
    }

    let last_value = sequence.last().unwrap();
    if has_non_zero {
        last_value + extrapolate(&steps)
    } else {
        *last_value
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut sum = 0;
    for line in input.lines() {
        let sequence = line
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
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
        assert_eq!("114", process(input)?);
        Ok(())
    }
}
