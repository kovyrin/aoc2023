use itertools::Itertools;
use std::{collections::HashMap, ops::Range};

use crate::custom_error::AocError;

#[derive(Debug)]
enum Rule {
    Condition {
        dim: char,
        op: char,
        value: u128,
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
        let value = cond[2..].parse::<u128>().unwrap();

        Self::Condition {
            dim: dimension,
            op,
            value,
            dest,
        }
    }

    fn dest(&self) -> String {
        match self {
            Self::Move(dest) | Self::Condition { dest, .. } => dest.clone(),
        }
    }

    fn supported_value_range(&self) -> Range<u128> {
        match self {
            Self::Move(_) => 1..4001,
            Self::Condition { op, value, .. } => match op {
                '<' => 1..*value,
                '>' => *value + 1..4001,
                _ => panic!("Unknown operator {} for {:?}", op, self),
            },
        }
    }

    // Returns a part range supported by the rule (a subset of the given range)
    fn supported_range(&self, part_range: &PartRange) -> Option<PartRange> {
        match self {
            Self::Move(_) => Some(part_range.clone()),
            Self::Condition { dim, .. } => {
                let mut supported_range = part_range.clone();

                // Get the current limits on the given dimension
                let dim_range = supported_range.dimensions.get(dim).unwrap();

                // If the current range is empty, the range is not supported by the rule
                if dim_range.start == dim_range.end {
                    return None;
                }

                // Create a range defined by the rule
                let op_range = self.supported_value_range();

                // Intersect the current range with the rule range
                let supported_value_range =
                    dim_range.start.max(op_range.start)..dim_range.end.min(op_range.end);

                // If the intersection is empty, the range is not supported by the rule
                if supported_value_range.start == supported_value_range.end {
                    return None;
                }

                // Update the range for the given dimension
                supported_range
                    .dimensions
                    .insert(*dim, supported_value_range);

                Some(supported_range)
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
}

#[derive(Debug, Clone)]
struct PartRange {
    dimensions: HashMap<char, Range<u128>>,
}

impl PartRange {
    fn full() -> Self {
        let mut dimensions = HashMap::new();
        for dim in ['x', 'm', 'a', 's'] {
            dimensions.insert(dim, 1..4001 as u128);
        }
        Self { dimensions }
    }

    fn size(&self) -> u128 {
        self.dimensions.values().map(|r| r.end - r.start).product()
    }

    fn exclude(&self, other: Self) -> Self {
        let mut dimensions = HashMap::new();
        for (dim, range) in &self.dimensions {
            let other_range = other.dimensions.get(dim).unwrap();
            let mut new_range = range.clone();

            if other_range.start > range.start {
                new_range.end = other_range.start;
            }
            if other_range.end < range.end {
                new_range.start = other_range.end;
            }

            dimensions.insert(*dim, new_range);
        }
        Self { dimensions }
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

    // Returns a number of different parts supported by the workflow from a given range
    fn supported_by_workflow(&self, name: &String, part_range: &PartRange) -> u128 {
        let mut part_range = part_range.clone();
        if name == "R" {
            return 0;
        }

        if name == "A" {
            return part_range.size();
        }

        let workflow = self.workflow_map.get(name).unwrap();
        let mut supported = 0u128;
        for rule in &workflow.rules {
            if let Some(supported_range) = rule.supported_range(&part_range) {
                supported += self.supported_by_workflow(&rule.dest(), &supported_range);
                part_range = part_range.exclude(supported_range);
            }
        }

        return supported;
    }

    fn new() -> Self {
        Self {
            workflows: Vec::new(),
            workflow_map: HashMap::new(),
        }
    }

    fn from_str(input: &str) -> Self {
        let mut sys = Self::new();
        for line in input.lines() {
            if line.is_empty() {
                break;
            }
            sys.add_workflow(Workflow::from_str(line));
        }
        sys
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let sys = System::from_str(input);
    let total_supported = sys.supported_by_workflow(&"in".to_string(), &PartRange::full());
    Ok(total_supported.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_supported_range() {
        let rule = Rule::from_str("a<2006:qkq");
        let part_range = PartRange::full();
        let supported_range = rule.supported_range(&part_range).unwrap();
        assert_eq!(1..2006, *supported_range.dimensions.get(&'a').unwrap());

        let rule = Rule::from_str("a>2006:qkq");
        let part_range = PartRange::full();
        let supported_range = rule.supported_range(&part_range).unwrap();
        assert_eq!(2007..4001, *supported_range.dimensions.get(&'a').unwrap());
    }

    #[test]
    fn test_range_size() {
        let part_range = PartRange::full();
        assert_eq!(4000 * 4000 * 4000 * 4000, part_range.size());

        let part_range = PartRange {
            dimensions: vec![('x', 1..5), ('m', 1..7), ('a', 1..9), ('s', 1..11)]
                .into_iter()
                .collect(),
        };
        assert_eq!(4 * 6 * 8 * 10, part_range.size());
    }

    #[test]
    fn test_supported_by_workflow_simple() {
        let mut sys = System::new();
        sys.add_workflow(Workflow::from_str("px{a<2006:R,A}"));
        let part_range = PartRange::full();
        let supported = sys.supported_by_workflow(&"px".to_string(), &part_range);
        // x = 2006..4001, m = 1..4001, a = 1..4001, s = 1..4001
        assert_eq!(1995 * 4000 * 4000 * 4000, supported);
    }

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
        assert_eq!("167409079868000", process(input)?);
        Ok(())
    }
}
