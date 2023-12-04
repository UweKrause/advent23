use std::cmp::max;
use std::fs::read_to_string;

#[derive(Clone, Debug)]
struct Engine {
    fields: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Engine {
    fn new() -> Engine {
        let width = 0;
        let height = 0;
        Self {
            fields: vec![vec!['.'; width]; height],
            width,
            height,
        }
    }

    fn add_row(&mut self, row_string: &str) {
        let mut row: Vec<char> = Vec::new();

        for c in row_string.chars() {
            row.push(c);
        }

        self.width = max(self.width, row.len());
        self.height += 1;
        self.fields.push(row);
    }

    fn from(s: String) -> Self {
        let mut engine = Engine::new();

        for line in s.lines() {
            engine.add_row(line)
        }

        engine
    }
}


fn main() {
    let engine = Engine::from(read_to_string("src/example").unwrap());

    println!("{:?}", engine);
    println!("{:?}", engine.fields[1][3]);
}
