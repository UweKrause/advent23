use std::collections::HashMap;
use std::fs::read_to_string;


fn main() {
    let (instructions, network)
        = parse_input(read_to_string("src/example3").unwrap());

    let mut instruction_cycle = instructions.chars().cycle();

    let mut step = 0;
    let mut current: Vec<Node> = network.nodes_start.clone();

    println!("{} {:?}", step, current);

    while current != network.nodes_end {
        step += 1;
        let instruction = instruction_cycle.next().unwrap();

        let mut next = Vec::new();
        for node in current {
            let x = network.get_node_by_label(node.get_label_by_direction(instruction));
            next.push(x);
        }

        current = next;
        println!("{} {} {:?}", step, instruction, current);
    }

    println!("{}", step);
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
    nodes_start: Vec<Node>,
    nodes_end: Vec<Node>,
}

impl Network {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            nodes_start: Vec::new(),
            nodes_end: Vec::new(),
        }
    }

    fn from(s: &str) -> Self {
        let mut network = Self::new();

        for node in s.split("\n") {
            network.insert(Node::from(node))
        }

        network
    }

    fn insert(&mut self, node: Node) {
        self.nodes.insert(node.name.to_string(), node.clone());

        if node.is_start_node() { self.nodes_start.push(node.clone()); }
        if node.is_end_node() { self.nodes_end.push(node.clone()); }
    }

    fn get_node_by_label(&self, label: String) -> Node {
        self.nodes.get(&label).unwrap().to_owned()
    }
}

#[derive(Debug, Clone)]
#[derive(PartialEq)]
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

    fn is_start_node(&self) -> bool { self.name.ends_with("A") }

    fn is_end_node(&self) -> bool { self.name.ends_with("Z") }
}




