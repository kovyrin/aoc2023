use crate::custom_error::AocError;
use num_integer::lcm;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug, Clone)]
struct StepGenerator {
    instructions: Vec<char>,
    current: usize,
    step_count: usize,
}

impl StepGenerator {
    fn next(&mut self) -> char {
        let step = self.instructions[self.current];
        self.current += 1;
        self.step_count += 1;
        if self.current >= self.instructions.len() {
            self.current = 0;
        }
        step
    }
}

#[derive(Debug)]
struct Ghost {
    current: String,
    steps: StepGenerator,
}

impl Ghost {
    fn from_node_name(node_name: &str, steps: &mut StepGenerator) -> Self {
        Ghost {
            current: node_name.to_string(),
            steps: steps.clone(),
        }
    }

    fn take_step(&mut self, nodes: &HashMap<&str, Node>) {
        let node = nodes.get(self.current.as_str()).unwrap();
        self.current = match self.steps.next() {
            'L' => node.left.clone(),
            'R' => node.right.clone(),
            _ => unreachable!(),
        };
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().trim();
    let mut steps = StepGenerator {
        instructions: instructions.chars().collect(),
        current: 0,
        step_count: 0,
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

    // find all the nodes where the name ends with 'A' and spawn a ghost for each
    let mut ghosts: Vec<Ghost> = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| Ghost::from_node_name(k, &mut steps))
        .collect();

    // Walk each ghost until each of them reaches a node that ends with 'Z'
    while !ghosts.iter().all(|g| g.current.ends_with('Z')) {
        for ghost in ghosts.iter_mut() {
            if !ghost.current.ends_with('Z') {
                ghost.take_step(&nodes);
            }
        }
    }

    // Find the least common multiple of all the ghost's step counts
    let mut lcm_steps = 1;
    for ghost in ghosts.iter() {
        lcm_steps = lcm(lcm_steps, ghost.steps.step_count);
    }

    Ok(lcm_steps.to_string())
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
// - 14616363770447 - correct
