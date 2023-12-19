use itertools::Itertools;
use std::collections::HashMap;

use crate::custom_error::AocError;

#[derive(Debug)]
enum Rule {
    Condition {
        dim: char,
        op: char,
        value: i64,
        dest: String,
    },
    Move(String),
}

impl Rule {
    fn from_str(s: &str) -> Self {
        if !s.contains(':') {
            return Self::Move(s.to_string());
        }

        // a<2006:qkq
        let mut parts = s.split(':');
        let cond = parts.next().unwrap();
        let dest = parts.last().unwrap().to_string();
        let dimension = cond.chars().nth(0).unwrap();
        let op = cond.chars().nth(1).unwrap();
        let value = cond[2..].parse::<i64>().unwrap();

        Self::Condition {
            dim: dimension,
            op,
            value,
            dest,
        }
    }

    fn matches(&self, part: &Part) -> bool {
        match self {
            Self::Move { .. } => true,
            Self::Condition { dim, op, value, .. } => {
                let part_value = part.dimensions.get(dim).unwrap();
                match op {
                    '<' => part_value < value,
                    '>' => part_value > value,
                    '=' => part_value == value,
                    _ => panic!("Unknown operator {}", op),
                }
            }
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    // px{a<2006:qkq,m>2090:A,rfg}
    fn from_str(input: &str) -> Self {
        let mut parts = input.trim().split('{');
        let name = parts.next().unwrap().to_string();
        let rules_str = parts.next().unwrap().strip_suffix('}').unwrap();
        let rules = rules_str
            .split(',')
            .map(|r| Rule::from_str(r))
            .collect_vec();
        Self { name, rules }
    }

    fn process(&self, part: &Part) -> &Rule {
        for rule in &self.rules {
            if rule.matches(part) {
                return rule;
            }
        }

        panic!(
            "No matching rule found for part {} in workflow {:?}",
            part, self.name
        );
    }
}

#[derive(Debug)]
struct Part {
    dimensions: HashMap<char, i64>,
}

impl Part {
    // {x=787,m=2655,a=1222,s=2876}
    fn from_str(l: &str) -> Self {
        let l = l
            .trim()
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap();
        let mut dimensions = HashMap::new();
        for part in l.split(',') {
            let mut parts = part.split('=');
            let dim = parts.next().unwrap().chars().nth(0).unwrap();
            let value = parts.next().unwrap().parse::<i64>().unwrap();
            dimensions.insert(dim, value);
        }
        Self { dimensions }
    }

    fn rating(&self) -> i64 {
        self.dimensions.values().sum()
    }
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{x={},m={},a={},s={}}}",
            self.dimensions[&'x'],
            self.dimensions[&'m'],
            self.dimensions[&'a'],
            self.dimensions[&'s']
        )
    }
}

struct System {
    workflows: Vec<String>,
    workflow_map: HashMap<String, Workflow>,
}

impl System {
    fn add_workflow(&mut self, workflow: Workflow) {
        let name = workflow.name.clone();
        self.workflows.push(name.clone());
        self.workflow_map.insert(name, workflow);
    }

    // Returns true if the part is accepted, false if rejected.
    fn process_part(&self, part: &Part) -> bool {
        let mut cur_name = "in".to_string();
        loop {
            let workflow = self.workflow_map.get(&cur_name).unwrap();
            match workflow.process(part) {
                Rule::Move(dest) | Rule::Condition { dest, .. } => {
                    cur_name = dest.clone();
                }
            }

            if cur_name == "A" {
                return true;
            }

            if cur_name == "R" {
                return false;
            }
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut sys = System {
        workflows: Vec::new(),
        workflow_map: HashMap::new(),
    };

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        sys.add_workflow(Workflow::from_str(line));
    }
    let parts = lines.map(|l| Part::from_str(l)).collect_vec();

    let mut total_rating = 0;
    for part in parts {
        if sys.process_part(&part) {
            total_rating += part.rating();
        }
    }

    Ok(total_rating.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
                     pv{a>1716:R,A}
                     lnx{m>1548:A,A}
                     rfg{s<537:gd,x>2440:R,A}
                     qs{s>3448:A,lnx}
                     qkq{x<1416:A,crn}
                     crn{x>2662:A,R}
                     in{s<1351:px,qqz}
                     qqz{s>2770:qs,m<1801:hdj,R}
                     gd{a>3333:R,R}
                     hdj{m>838:A,pv}

                     {x=787,m=2655,a=1222,s=2876}
                     {x=1679,m=44,a=2067,s=496}
                     {x=2036,m=264,a=79,s=2244}
                     {x=2461,m=1339,a=466,s=291}
                     {x=2127,m=1623,a=2188,s=1013}";
        assert_eq!("19114", process(input)?);
        Ok(())
    }
}
