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

impl StepGenerator {
    fn next(&mut self) -> char {
        let step = self.instructions[self.current];
        self.current += 1;
        if self.current >= self.instructions.len() {
            self.current = 0;
        }
        step
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

    // find all the nodes where the name ends with 'A'
    let mut current_nodes: Vec<&str> = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| *k)
        .collect();
    let mut step_count = 0;

    loop {
        // if all the current nodes end with 'Z', we're done
        if current_nodes.iter().all(|n| n.ends_with('Z')) {
            break;
        }

        let step = steps.next();
        step_count += 1;
        if step_count % 1000 == 0 {
            println!("step_count: {}", step_count);
        }

        for current in current_nodes.iter_mut() {
            let node = nodes.get(current).unwrap();
            match step {
                'L' => *current = &node.left,
                'R' => *current = &node.right,
                _ => unreachable!(),
            }
        }
    }

    Ok(step_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "LR

                     11A = (11B, XXX)
                     11B = (XXX, 11Z)
                     11Z = (11B, XXX)
                     22A = (22B, XXX)
                     22B = (22C, 22C)
                     22C = (22Z, 22Z)
                     22Z = (22B, 22B)
                     XXX = (XXX, XXX)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}

// Submissions:
// - 1000000000 - too low
// - 10000000000 - too low
// - TBD
