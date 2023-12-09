use itertools::Itertools;

use crate::custom_error::AocError;

pub fn extrapolate(sequence: &Vec<i64>) -> i64 {
    println!("extrapolate({:?})", sequence);
    let mut steps = Vec::with_capacity(sequence.len() - 1);
    for i in 1..sequence.len() {
        steps.push(sequence[i] - sequence[i - 1]);
    }
    println!("- steps: {:?}", steps);
    if steps.iter().all_equal() && steps[0] == 0 {
        let next = sequence[sequence.len() - 1] + steps[1];
        println!("- next for {:?} is {}", sequence, next);
        return next;
    }

    let next = sequence[sequence.len() - 1] + extrapolate(&steps);
    println!("- next for {:?} is {}", sequence, next);
    next
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
        let next_number = extrapolate(&sequence);
        println!("{:?} -> {}\n", sequence, next_number);
        sum += next_number;
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
