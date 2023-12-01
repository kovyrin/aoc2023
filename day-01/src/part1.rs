use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut sum = 0;

    input.lines().for_each(|line| {
        let mut digit1 = None;
        let mut digit2 = None;

        line.chars().for_each(|c| {
            if c.is_numeric() {
                if digit1.is_none() {
                    digit1 = Some(c);
                } else {
                    digit2 = Some(c);
                }
            }
        });

        if digit2.is_none() {
            digit2 = digit1;
        }

        let number =
            digit1.unwrap().to_digit(10).unwrap() * 10 + digit2.unwrap().to_digit(10).unwrap();
        sum += number;
    });

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#;
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
