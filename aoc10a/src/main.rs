use std::collections::HashMap;
use std::fs::read_to_string;
use crate::Direction::{East, North, South, West};

fn main() {
    let grid = Grid::from(read_to_string("src/example2").unwrap());

    println!("{:?}", &grid.connections_by_position(grid.start.x, grid.start.y));

    dbg!(grid.get_neighbours(grid.start.clone()));
}

#[derive(Debug)]
struct Grid {
    tiles: HashMap<usize, HashMap<usize, Tile>>,
    start: Tile,
}

impl Grid {
    fn new() -> Self {
        let tiles = HashMap::new();
        let start = Tile::new();
        Self { tiles, start }
    }

    fn from(s: String) -> Self {
        let mut grid = Grid::new();

        for (x, input_row) in s.trim().split("\n").enumerate() {
            let mut row = HashMap::new();
            for (y, char) in input_row.chars().enumerate() {
                let connects = Grid::connections_by_char(char);
                let tile = Tile { x, y, char, connects };
                row.insert(y, tile.clone());

                if char == 'S' { grid.start = tile.clone() }
            }

            grid.tiles.insert(x, row);
        }

        grid.start.connects.append(&mut grid.connections_by_position(
            grid.start.x,
            grid.start.y,
        ));

        grid
    }

    fn connections_by_char(char: char) -> Vec<Direction> {
        /*
        | is a vertical pipe connecting north and south.
        - is a horizontal pipe connecting east and west.
        L is a 90-degree bend connecting north and east.
        J is a 90-degree bend connecting north and west.
        7 is a 90-degree bend connecting south and west.
        F is a 90-degree bend connecting south and east.
        . is ground; there is no pipe in this tile.
        S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
         */

        match char {
            '|' => vec![North, South],
            '-' => vec![East, West],
            'L' => vec![North, East],
            'J' => vec![North, West],
            '7' => vec![South, West],
            'F' => vec![South, East],
            '.' => vec![],
            'S' => vec![], // Empty for now, needs to be filled later!
            _ => panic!()
        }
    }

    fn connections_by_position(&self, x: usize, y: usize) -> Vec<Direction> {
        // for all directions that are next to this position, check if there is a tile.
        // iff there is a tile, check if that tile connects to this.
        // iff it does, this position also connects to that position.

        let mut connections = Vec::new();

        if x > 0 && self.get(x - 1, y).connects_to(South) { connections.push(North) }
        if y > 0 && self.get(x, y - 1).connects_to(East) { connections.push(West) }
        if x < self.height() && self.get(x + 1, y).connects_to(North) { connections.push(South) }
        if y < self.width() && self.get(x, y + 1).connects_to(West) { connections.push(East) }

        connections
    }

    fn size(&self) -> usize {
        assert_eq!(self.width(), self.height());
        self.width()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn width(&self) -> usize {
        assert!(self.height() > 0);
        self.tiles.get(&0).unwrap().len()
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles.get(&x).unwrap().get(&y).unwrap().to_owned()
    }

    fn get_neighbours(&self, tile: Tile) -> Vec<Tile> {
        let mut neighbours = Vec::new();

        let x = tile.x;
        let y = tile.y;

        dbg!(&tile.connects);

        for connects in tile.connects {
            match connects {
                North => neighbours.push(self.get(x - 1, y)),
                South => neighbours.push(self.get(x + 1, y)),
                East => neighbours.push(self.get(x, y + 1)),
                West => neighbours.push(self.get(x, y - 1)),
            }
        }

        neighbours
    }
}


#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Tile {
    x: usize,
    y: usize,
    char: char,
    connects: Vec<Direction>,
}

impl Tile {
    fn new() -> Self { Self { x: 0, y: 0, char: '.', connects: vec![] } }

    fn connects_to(&self, direction: Direction) -> bool {
        self.connects.contains(&direction)
    }
}


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Direction { North, South, East, West }
