use crate::custom_error::AocError;

fn aoc_hash(input: &str) -> u64 {
    let mut hash = 0;
    for c in input.chars() {
        hash = ((hash + c as u64) * 17) % 256;
    }
    return hash;
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let result = input.trim().split(",").map(|s| aoc_hash(s)).sum::<u64>();
    return Ok(result.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(52, aoc_hash("HASH"));
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("1320", process(input)?);
        Ok(())
    }
}

// Submissions:
// 513166 - too low
