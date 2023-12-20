use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Signal {
    High,
    Low,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum NodeType {
    Broadcast,
    FlipFlop(bool),
    Conjunct,
}

#[derive(Debug)]
struct Node {
    name: String,
    node_type: NodeType,
    out_conns: Vec<String>,
    in_conns: Vec<String>,
}

impl Node {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split(" -> ");
        let name_with_type = parts.next().unwrap();
        let conns = parts.next().unwrap();
        let node_type = match name_with_type.chars().nth(0).unwrap() {
            '&' => NodeType::Conjunct,
            '%' => NodeType::FlipFlop(false), // Off by default
            _ => NodeType::Broadcast,
        };
        let conns = conns.split(", ").map(|s| s.to_string()).collect();
        let name = match node_type {
            NodeType::Broadcast => name_with_type.to_string(),
            _ => name_with_type[1..].to_string(),
        };

        Self {
            name,
            node_type,
            out_conns: conns,
            in_conns: vec![],
        }
    }

    fn process(&mut self, signal_type: Signal) -> Vec<(String, Signal)> {
        match self.node_type {
            NodeType::Broadcast => self.process_broadcast(signal_type),
            NodeType::FlipFlop(on) => self.process_flipflop(on, signal_type),
            NodeType::Conjunct => self.process_conjunct(signal_type),
        }
    }

    fn process_broadcast(&self, signal_type: Signal) -> Vec<(String, Signal)> {
        self.out_conns
            .iter()
            .map(|conn| (conn.clone(), signal_type))
            .collect()
    }

    fn process_flipflop(&mut self, on: bool, signal_type: Signal) -> Vec<(String, Signal)> {
        if signal_type == Signal::Low {
            self.node_type = NodeType::FlipFlop(!on);
            let new_pulse = if on { Signal::Low } else { Signal::High };
            self.process_broadcast(new_pulse)
        } else {
            vec![]
        }
    }

    fn process_conjunct(&self, signal_type: Signal) -> Vec<(String, Signal)> {
        vec![]
    }
}

#[derive(Debug)]
struct Network {
    nodes: HashMap<String, Node>,
    high_pulses: u64,
    low_pulses: u64,
}

impl Network {
    fn from_str(s: &str) -> Self {
        let mut nodes: HashMap<String, Node> = HashMap::new();
        let mut in_conns: HashMap<String, Vec<String>> = HashMap::new();
        for line in s.lines() {
            let node = Node::from_str(line.trim());
            for conn in &node.out_conns {
                in_conns
                    .entry(conn.clone())
                    .or_default()
                    .push(node.name.clone());
            }
            nodes.insert(node.name.clone(), node);
        }

        for node in nodes.values_mut() {
            node.in_conns = in_conns.remove(&node.name).unwrap_or_default();
        }

        Self {
            nodes,
            high_pulses: 0,
            low_pulses: 0,
        }
    }

    fn score(&self) -> u64 {
        self.high_pulses * self.low_pulses
    }

    // Simulates a single pulse sent through the network, starting at the broadcaster module.
    fn press_button(&mut self) {
        let mut signal_queue = VecDeque::new();
        signal_queue.push_back(("broadcaster".to_string(), Signal::Low));

        while let Some((node_name, signal_type)) = signal_queue.pop_front() {
            match signal_type {
                Signal::High => self.high_pulses += 1,
                Signal::Low => self.low_pulses += 1,
            }

            let node = self.nodes.get_mut(&node_name).unwrap();
            println!(
                "Processing node {} with signal {:?}: {:?}",
                node_name, signal_type, node
            );

            let outgoing_signals = node.process(signal_type);
            println!("Outgoing signals: {:?}", outgoing_signals);

            for (conn, signal_type) in outgoing_signals {
                signal_queue.push_back((conn, signal_type));
            }
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut net = Network::from_str(input);
    println!("{:#?}", net);

    net.press_button();

    Ok(net.score().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "broadcaster -> a, b, c
                     %a -> b
                     %b -> c
                     %c -> inv
                     &inv -> a";
        assert_eq!("32000000", process(input)?);
        Ok(())
    }
}
