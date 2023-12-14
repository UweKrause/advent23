use std::collections::HashMap;
use std::fs::read_to_string;


fn main() {
    let (instructions, network)
        = parse_input(read_to_string("src/example2").unwrap());

    let mut steps = 0;

    let mut last = network.get_by_label("AAA".to_string());

    for direction in instructions.chars().cycle() {
        steps += 1;
        let current_label = last.unwrap().get_by_direction(direction);
        last = network.get_by_label(current_label);
        if last.unwrap().name == "ZZZ" {
            break;
        }
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
            network.push(Node::from(node))
        }

        network
    }

    fn push(&mut self, node: Node) {
        self.nodes.insert((&node.name).to_string(), node);
    }

    fn get_by_label(&self, label: String) -> Option<&Node> {
        self.nodes.get(&label)
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn from(node_string: &str) -> Self {
        Self {
            name: node_string[0..=2].to_string(),
            left: node_string[7..=9].to_string(),
            right: node_string[12..=14].to_string(),
        }
    }

    fn get_by_direction(&self, direction: char) -> String {
        match direction {
            'L' => self.left.to_string(),
            'R' => self.right.to_string(),
            _ => panic!()
        }
    }
}




