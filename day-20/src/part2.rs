use fxhash::FxHashMap;
use std::collections::{HashMap, VecDeque};

use crate::custom_error::AocError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum SignalType {
    High,
    Low,
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
    fn set_incoming(&mut self, _: &Vec<String>) {}
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
    in_state: FxHashMap<String, bool>,
}

impl ConjunctNode {
    fn new(name: &str, conns: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            out_conns: conns,
            in_state: FxHashMap::default(),
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
    nodes: FxHashMap<String, BoxedNode>,
    pulse_counts: FxHashMap<SignalType, u64>,
}

impl Network {
    fn from_str(s: &str) -> Self {
        let mut nodes: FxHashMap<String, BoxedNode> = FxHashMap::default();
        let mut in_conns: FxHashMap<String, Vec<String>> = FxHashMap::default();
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
            pulse_counts: FxHashMap::default(),
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

    // Simulates a single pulse sent through the network, starting at the broadcaster module.
    fn press_button_and_measure(&mut self, probe_names: &Vec<&str>) -> Vec<String> {
        let mut signal_queue = VecDeque::new();
        signal_queue.push_back(Signal {
            src: "button".to_string(),
            dst: "broadcaster".to_string(),
            signal_type: SignalType::Low,
        });

        let mut triggered_probes = Vec::new();
        while let Some(signal) = signal_queue.pop_front() {
            self.pulse_counts
                .entry(signal.signal_type)
                .and_modify(|c| *c += 1)
                .or_insert(1);

            if signal.signal_type == SignalType::High && probe_names.contains(&signal.src.as_str())
            {
                triggered_probes.push(signal.src.clone());
            }

            if let Some(dst_node) = self.nodes.get_mut(&signal.dst) {
                let outgoing_signals = dst_node.process_signal(&signal);
                signal_queue.extend(outgoing_signals);
            }
        }

        return triggered_probes;
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut net = Network::from_str(input);

    let mut presses = 0;
    let mut probes = vec!["kf", "qk", "zs", "kr"];
    let mut loop_sizes = HashMap::new();

    while probes.len() > 0 {
        let triggered = net.press_button_and_measure(&probes);
        presses += 1;

        for probe in triggered {
            println!("Loop detected for probe: {} => {}", probe, presses);
            probes.retain(|p| *p != probe);
            loop_sizes.insert(probe, presses);
        }
    }

    let mut lcm = 1u64;
    for size in loop_sizes.values() {
        lcm = num_integer::lcm(lcm, *size);
    }

    Ok(lcm.to_string())
}

// Submissions:
// 179137411 - too low
// 1241252064 - too low
// 231897990075517 - correct!
