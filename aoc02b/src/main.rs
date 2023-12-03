use std::cmp::{max};
use std::fs::read_to_string;

fn main() {
    let power_sum: u32 = read_to_string("src/example").unwrap().lines()
        .map(|game_line| Game::from(game_line))
        .map(|game| game.minimal_bag())
        .map(|minimal_bag| minimal_bag.power())
        .sum();

    println!("{}", power_sum); // 2286
}

#[derive(Debug)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    _id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn from(game_as_str: &str) -> Self {
        let id: u32;
        let mut rounds: Vec<Round> = Vec::new();

        // parse game id
        let line_split: Vec<&str> = game_as_str.split(": ").collect();
        let line_game = line_split[0];
        let line_rounds = line_split[1];

        id = line_game.split_whitespace().collect::<Vec<_>>()[1].parse().unwrap();

        // extract rounds
        for round_as_str in line_rounds.split("; ").collect::<Vec<_>>() {
            rounds.push(Round::from(round_as_str));
        }

        Self { _id: id, rounds }
    }

    fn minimal_bag(&self) -> Bag {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for round in &self.rounds {
            red = max(red, round.red);
            green = max(green, round.green);
            blue = max(blue, round.blue);
        }

        Bag { red, green, blue }
    }
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn from(round_as_string: &str) -> Self {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;


        for cubes_as_string in round_as_string.split(", ").collect::<Vec<_>>() {
            let cubes_as_string_split: Vec<_> = cubes_as_string.split_whitespace().collect();
            let cubes_color: &str = cubes_as_string_split[1];
            let cubes_count: u32 = cubes_as_string_split[0].parse().unwrap();

            match cubes_color {
                "red" => red = cubes_count,
                "green" => green = cubes_count,
                "blue" => blue = cubes_count,
                _ => ()
            }
        }

        Self { red, green, blue }
    }
}




