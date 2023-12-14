use std::collections::HashMap;
use std::fs::read_to_string;


fn main() {
    let (instructions, network)
        = parse_input(read_to_string("src/example2").unwrap());

    let label_start = "AAA";
    let label_end = "ZZZ";

    let mut instruction_cycle = instructions.chars().cycle();

    let mut steps = 0;
    let mut current = network.get_node_by_label(label_start.to_string());

    while current.name != label_end {
        steps += 1;
        current = network.get_node_by_label(current.get_label_by_direction(instruction_cycle.next().unwrap()));
    }

    println!("{}", steps);
}

fn parse_input(s: String) -> (String, Network) {
    let (input_instructions, input_nodes)
        = s.split_once("\n").unwrap();

    let instructions = input_instructions.to_string();
    let network = Network::from(input_nodes.trim());

    return (instructions, network);
}

#[derive(Debug)]
struct Network {
    nodes: HashMap<String, Node>,
}

impl Network {
    fn new() -> Self { Self { nodes: HashMap::new() } }

    fn from(s: &str) -> Self {
        let mut network = Self::new();

        for node in s.split("\n") {
            network.insert(Node::from(node))
        }

        network
    }

    fn insert(&mut self, node: Node) {
        self.nodes.insert((&node.name).to_string(), node);
    }

    fn get_node_by_label(&self, label: String) -> &Node {
        self.nodes.get(&label).unwrap()
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn from(node_str: &str) -> Self {
        Self {
            name: node_str[0..=2].to_string(),
            left: node_str[7..=9].to_string(),
            right: node_str[12..=14].to_string(),
        }
    }

    fn get_label_by_direction(&self, direction: char) -> String {
        match direction {
            'L' => self.left.clone(),
            'R' => self.right.clone(),
            _ => panic!()
        }
    }
}




