use std::collections::{HashMap, VecDeque};

use crate::custom_error::AocError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum SignalType {
    High,
    Low,
}

impl std::fmt::Display for SignalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalType::High => write!(f, "high"),
            SignalType::Low => write!(f, "low"),
        }
    }
}

#[derive(Debug)]
struct Signal {
    src: String,
    dst: String,
    signal_type: SignalType,
}

impl Signal {
    fn send(&self, conn: &str, signal_type: SignalType) -> Self {
        Self {
            src: self.dst.clone(),
            dst: conn.to_string(),
            signal_type,
        }
    }

    fn broadcast(&self, conns: &[String], new_type: Option<SignalType>) -> Vec<Self> {
        let new_type = new_type.unwrap_or(self.signal_type);
        conns.iter().map(|conn| self.send(conn, new_type)).collect()
    }
}

//-----------------------------------------------------------------------------
trait Node {
    fn name(&self) -> &str;
    fn out_conns(&self) -> &[String];
    fn set_incoming(&mut self, incoming: &Vec<String>);
    fn process_signal(&mut self, signal: &Signal) -> Vec<Signal>;
}

//-----------------------------------------------------------------------------
#[derive(Debug)]
struct BroadcastNode {
    name: String,
    out_conns: Vec<String>,
}
impl BroadcastNode {
    fn new(name: &str, conns: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            out_conns: conns,
        }
    }
}

impl Node for BroadcastNode {
    fn name(&self) -> &str {
        &self.name
    }

    fn out_conns(&self) -> &[String] {
        &self.out_conns
    }

    fn set_incoming(&mut self, _: &Vec<String>) {}

    fn process_signal(&mut self, signal: &Signal) -> Vec<Signal> {
        signal.broadcast(&self.out_conns, None)
    }
}

//-----------------------------------------------------------------------------
#[derive(Debug)]
struct FlipFlopNode {
    name: String,
    out_conns: Vec<String>,
    on: bool,
}

impl FlipFlopNode {
    fn new(name: &str, conns: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            out_conns: conns,
            on: false,
        }
    }
}

impl Node for FlipFlopNode {
    fn name(&self) -> &str {
        &self.name
    }

    fn out_conns(&self) -> &[String] {
        &self.out_conns
    }

    fn set_incoming(&mut self, _: &Vec<String>) {}

    fn process_signal(&mut self, signal: &Signal) -> Vec<Signal> {
        if signal.signal_type == SignalType::High {
            return vec![];
        }

        let out_signal = if self.on {
            SignalType::Low
        } else {
            SignalType::High
        };
        self.on = !self.on;

        signal.broadcast(&self.out_conns, Some(out_signal))
    }
}

//-----------------------------------------------------------------------------
#[derive(Debug)]
struct ConjunctNode {
    name: String,
    out_conns: Vec<String>,
    in_state: HashMap<String, bool>,
}

impl ConjunctNode {
    fn new(name: &str, conns: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            out_conns: conns,
            in_state: HashMap::new(),
        }
    }
}

impl Node for ConjunctNode {
    fn name(&self) -> &str {
        &self.name
    }

    fn out_conns(&self) -> &[String] {
        &self.out_conns
    }

    fn set_incoming(&mut self, in_names: &Vec<String>) {
        for name in in_names {
            self.in_state.insert(name.to_string(), false);
        }
    }

    fn process_signal(&mut self, signal: &Signal) -> Vec<Signal> {
        let state_for_input = self.in_state.get_mut(&signal.src).unwrap();
        *state_for_input = signal.signal_type == SignalType::High;

        let out_signal = if self.in_state.values().all(|v| *v) {
            SignalType::Low
        } else {
            SignalType::High
        };

        signal.broadcast(&self.out_conns, Some(out_signal))
    }
}

type BoxedNode = Box<dyn Node>;

struct Network {
    nodes: HashMap<String, BoxedNode>,
    pulse_counts: HashMap<SignalType, u64>,
}

impl Network {
    fn from_str(s: &str) -> Self {
        let mut nodes: HashMap<String, BoxedNode> = HashMap::new();
        let mut in_conns: HashMap<String, Vec<String>> = HashMap::new();
        for line in s.lines() {
            let node = Self::node_from_str(line.trim());
            for conn in node.out_conns() {
                in_conns
                    .entry(conn.clone())
                    .or_default()
                    .push(node.name().to_string());
            }
            nodes.insert(node.name().to_string(), node);
        }

        for node in nodes.values_mut() {
            let incoming_conns = in_conns.remove(node.name()).unwrap_or_default();
            node.set_incoming(&incoming_conns);
        }

        Self {
            nodes,
            pulse_counts: HashMap::new(),
        }
    }

    fn node_from_str(s: &str) -> BoxedNode {
        let mut parts = s.split(" -> ");
        let name_with_type = parts.next().unwrap();
        let node_type = name_with_type.chars().nth(0).unwrap();

        let conns = parts.next().unwrap();
        let conns = conns.split(", ").map(|s| s.to_string()).collect();

        match node_type {
            '&' => Box::new(ConjunctNode::new(&name_with_type[1..], conns)),
            '%' => Box::new(FlipFlopNode::new(&name_with_type[1..], conns)),
            _ => Box::new(BroadcastNode::new(&name_with_type, conns)),
        }
    }

    fn score(&self) -> u64 {
        self.pulse_counts.values().product()
    }

    // Simulates a single pulse sent through the network, starting at the broadcaster module.
    fn press_button(&mut self) {
        let mut signal_queue = VecDeque::new();
        signal_queue.push_back(Signal {
            src: "button".to_string(),
            dst: "broadcaster".to_string(),
            signal_type: SignalType::Low,
        });

        while let Some(signal) = signal_queue.pop_front() {
            self.pulse_counts
                .entry(signal.signal_type)
                .and_modify(|c| *c += 1)
                .or_insert(1);

            if let Some(dst_node) = self.nodes.get_mut(&signal.dst) {
                let outgoing_signals = dst_node.process_signal(&signal);
                signal_queue.extend(outgoing_signals);
            }
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut net = Network::from_str(input);
    for _ in 0..1000 {
        net.press_button();
    }
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

    #[test]
    fn test_process_harder() {
        let input = "broadcaster -> a
                     %a -> inv, con
                     &inv -> b
                     %b -> con
                     &con -> output";
        assert_eq!("11687500", process(input).unwrap());
    }

    #[test]
    fn test_single_and() {
        let mut node = ConjunctNode::new("and", vec!["output".to_string()]);
        node.set_incoming(&vec!["a".to_string()]);

        let high = Signal {
            src: "a".to_string(),
            dst: "and".to_string(),
            signal_type: SignalType::High,
        };

        let low = Signal {
            src: "a".to_string(),
            dst: "and".to_string(),
            signal_type: SignalType::Low,
        };

        let res = node.process_signal(&low);
        println!("{:?}", res);

        let res = node.process_signal(&low);
        println!("{:?}", res);

        let res = node.process_signal(&low);
        println!("{:?}", res);

        let res = node.process_signal(&high);
        println!("{:?}", res);

        let res = node.process_signal(&high);
        println!("{:?}", res);

        let res = node.process_signal(&high);
        println!("{:?}", res);
    }
}

// Submissions:
// 672744417 - too low
// 697264874 - too low
// 697264974 - too low
// 739960225 - correct
