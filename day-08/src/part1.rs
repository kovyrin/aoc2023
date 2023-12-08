use std::collections::HashMap;

use crate::custom_error::AocError;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
struct StepGenerator {
    instructions: Vec<char>,
    current: usize,
}

impl Iterator for StepGenerator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let step = self.instructions[self.current];
        self.current += 1;
        if self.current >= self.instructions.len() {
            self.current = 0;
        }
        Some(step)
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().trim();
    let mut steps = StepGenerator {
        instructions: instructions.chars().collect(),
        current: 0,
    };

    lines.next();

    let mut nodes = HashMap::new();
    for line in lines {
        let line = line.trim();
        let mut parts = line.split(" = ");
        let node_name = parts.next().unwrap().trim();
        let adjacent = parts.next().unwrap().trim();
        let mut adjacent = adjacent
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(", ")
            .map(|s| s.to_string());
        let left = adjacent.next().unwrap().clone();
        let right = adjacent.next().unwrap().clone();

        nodes.insert(node_name, Node { left, right });
    }

    let mut current = "AAA";
    let mut step_count = 0;
    while current != "ZZZ" {
        let node = nodes.get(current).unwrap();
        let step = steps.next().unwrap();
        match step {
            'L' => current = &node.left,
            'R' => current = &node.right,
            _ => unreachable!(),
        }
        step_count += 1;
    }

    Ok(step_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "RL

                     AAA = (BBB, CCC)
                     BBB = (DDD, EEE)
                     CCC = (ZZZ, GGG)
                     DDD = (DDD, DDD)
                     EEE = (EEE, EEE)
                     GGG = (GGG, GGG)
                     ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input = "LLR

                     AAA = (BBB, BBB)
                     BBB = (AAA, ZZZ)
                     ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
