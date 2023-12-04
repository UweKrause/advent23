use std::cmp::max;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::num::ParseIntError;

#[derive(Clone, Debug)]
struct Engine {
    fields: Vec<Vec<char>>,
    width: usize,
    height: usize,
    symbols: HashSet<char>,
}

impl Engine {
    fn new() -> Engine {
        let width = 0;
        let height = 0;
        Self {
            fields: vec![vec!['.'; width]; height],
            width,
            height,
            symbols: HashSet::new(),
        }
    }

    fn from(s: String) -> Self {
        let mut engine = Engine::new();
        for line in s.lines() { engine.add_row(line) }
        return engine;
    }

    fn add_row(&mut self, row_string: &str) {
        let mut row: Vec<char> = Vec::new();

        for c in row_string.chars() {
            row.push(c);
            if !c.is_numeric() && c != '.' { self.symbols.insert(c); }
        }

        // Assumption: Every line has the same length
        self.width = max(self.width, row.len());
        self.height += 1;
        self.fields.push(row);
    }


    fn get_neighbors(&self, row: usize, pos: usize) -> Vec<char> {
        let mut neighbors: Vec<char> = Vec::new();

        let neighbor_window: [(isize, isize); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1), /*(0,0)*/ (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];

        for (window_row, window_pos) in neighbor_window {
            let neighbor_row = row as isize + window_row;
            let neighbor_pos = pos as isize + window_pos;

            if neighbor_row >= 0
                && neighbor_pos >= 0
                && neighbor_row < self.height as isize
                && neighbor_pos < self.width as isize
            {
                let n = self.fields[neighbor_row as usize][neighbor_pos as usize];
                if n != '.' { neighbors.push(n); }
            }
        }

        return neighbors;
    }

    fn get_numbers(&self) -> Vec<Number> {
        // Extracts all Numbers (as defined) from the Engine.
        // Numbers are build one char at a time:
        // From left to right, upper to lower check every char if it is a digit.
        // If it is a digit
        // - append the char to the Number currently build
        // - append the neighbouring chars of this field to the list of neighbours
        // If the current character is not a digit, this might be the end of a Number being build.
        // The number can be added to the list of numbers.

        let mut numbers: Vec<Number> = Vec::new();

        // collectors for the build up of the Number
        let mut chars: Vec<char> = Vec::new();
        let mut neighbors: Vec<char> = Vec::new();

        for r in 0..self.height {
            for p in 0..self.width {
                let c = self.fields[r][p];

                if c.is_digit(10) {
                    chars.push(c);
                    neighbors.append(&mut self.get_neighbors(r, p));
                } else {
                    if chars.len() > 0 {
                        // create a new Number from the collected characters
                        numbers.push(Number { chars: chars.clone(), neighbors: neighbors.clone() });

                        // reset the collectors
                        chars.clear();
                        neighbors.clear();
                    }
                }
            }
        }
        return numbers;
    }

    fn is_part_number(&self, number: &Number) -> bool {
        // a number is a part number iff any of its neighboring chars is a symbol
        number.neighbors.iter()
            .any(|x| self.symbols.contains(x))
    }
}

struct Number {
    chars: Vec<char>,
    neighbors: Vec<char>,
}

impl Number {
    fn to_digit(&self) -> Result<u32, ParseIntError> {
        let mut number_string = String::new();
        for c in self.chars.clone().into_iter() { number_string.push(c); }
        return number_string.parse();
    }
}


fn main() {
    let engine = Engine::from(read_to_string("src/example").unwrap());

    let pn_sum: u32 = engine.get_numbers().iter()
        .filter(|n| engine.is_part_number(n))
        .map(|n| n.to_digit().unwrap())
        .sum();

    println!("{}", pn_sum); // 4361
}
