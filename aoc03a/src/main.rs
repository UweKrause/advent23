use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

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

    fn add_row(&mut self, row_string: &str) {
        let mut row: Vec<char> = Vec::new();

        for c in row_string.chars() {
            row.push(c);

            if !c.is_numeric() && c != '.' {
                self.symbols.insert(c);
            }
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

    fn field(&self, row: usize, pos: usize) -> char {
        self.fields[row][pos]
    }

    fn neighbors(&self, row: usize, pos: usize) -> Vec<char> {
        let mut neighbors: Vec<char> = Vec::new();

        let neighbor_window: [(isize, isize); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1), /*(0,0)*/ (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];

        for (window_row, window_pos) in neighbor_window {
            let neighbor_row = row as isize + window_row;
            let neighbor_pos = pos as isize + window_pos;

            if neighbor_row >= 0 && neighbor_pos >= 0
                && neighbor_row < self.height as isize && neighbor_pos < self.width as isize
            {
                let n = self.field(neighbor_row as usize, neighbor_pos as usize);
                if n != '.' { neighbors.push(n) }
            }
        }

        neighbors
    }

    fn numbers(&self) -> (Vec<Vec<char>>, Vec<Vec<(usize, usize)>>) {
        let mut numbers: Vec<Vec<char>> = Vec::new();
        let mut numbers_positions: Vec<Vec<(usize, usize)>> = Vec::new();


        for r in 0..self.height {
            let mut number: Vec<char> = Vec::new();
            let mut number_positions: Vec<(usize, usize)> = Vec::new();


            for p in 0..self.width {
                let c = self.field(r, p);

                if c.is_digit(10) {
                    number.push(c);
                    number_positions.push((r, p))
                } else {
                    if number.len() > 0 {
                        numbers.push(number.clone());
                        numbers_positions.push(number_positions.clone());
                        number.clear();
                        number_positions.clear();
                    }
                }
            }
        }
        (numbers, numbers_positions)
    }

    fn partnumbers(
        numbers: Vec<Vec<char>>,
        positions: Vec<Vec<(usize, usize)>>,
    ) {
        let mut partnumbers: Vec<i32> = Vec::new();

        for (number, position) in numbers.iter().zip(positions) {
            println!("{:?} {:?}", number, position);
        }
    }

    fn is_partnumber(&self, positions: Vec<(usize, usize)>) -> bool {
        let mut partnumber = true;

        // ugly, but works for now...
        // ToDo: clean up this mess
        let mut neighbors: Vec<char> = Vec::new();
        for (r, p) in positions.into_iter() {
            neighbors.append(&mut self.neighbors(r, p))
        }
        for n in neighbors {
            partnumber = partnumber && !self.symbols.contains(&n);
        }

        !partnumber
    }
}


fn main() {
    let engine = Engine::from(read_to_string("src/example").unwrap());

    // println!("{:?}", engine);
    // println!("{:?}", engine.symbols);
    // println!("{:?}", engine.field(1, 3));
    // println!("{:?}", engine.neighbors(1, 3));
    // println!("{:?}", engine.neighbors(0, 0));


    let (numbers, positions) = engine.numbers();

    let mut pn_sum = 0;

    for (number, position) in numbers.iter().zip(positions) {
        println!("{:?} {:?} {:?}",
                 &number,
                 &position,
                 engine.is_partnumber(position.clone())
        );

        if engine.is_partnumber(position.clone()) {
            let mut number_string: String = "".to_string();

            for c in number {
                number_string.push(*c)
            }

            let number_value: usize = number_string.clone().parse().unwrap();

            pn_sum += number_value;
        }
    }

    println!("{}", pn_sum);
}
