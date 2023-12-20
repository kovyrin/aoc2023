use std::collections::HashMap;

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
    fn type_char(&self) -> char;
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
    fn type_char(&self) -> char {
        '#'
    }

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
    fn type_char(&self) -> char {
        '%'
    }

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
    fn type_char(&self) -> char {
        '&'
    }

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

        Self { nodes }
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
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let net = Network::from_str(input);

    println!("strict digraph {{");

    for node in net.nodes.values() {
        let mut attrs = vec![
            format!("label=\"{} {}\"", node.type_char(), node.name()),
            format!("shape=box"),
        ];

        if node.name() == "rx" {
            attrs.push(format!("rank=max"));
        } else if node.name() == "broadcaster" {
            attrs.push(format!("rank=min"));
        };

        println!(
            " {} [{}];",
            node.name(),
            attrs.iter().map(|s| format!("{} ", s)).collect::<String>()
        );
    }

    for node in net.nodes.values() {
        for conn in node.out_conns() {
            println!(" {} -> {};", node.name(), conn);
        }
    }

    println!("}}");

    Ok("".to_string())
}
