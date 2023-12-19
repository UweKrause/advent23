use std::fs::read_to_string;
use Color::{Blue, Green, Red};

fn main() {
    let bag = Bag { red: 12, green: 13, blue: 14 };

    let id_sum =
        read_to_string("src/input")
            .unwrap()
            .lines()
            // "Game 50: 9 red; 5 green, 2 blue, 10 red; 5 red, 1 green\n"
            .map(Game::from)
            .filter(|game| game.possible(&bag))
            .map(|game| game.id)
            .sum::<u32>();

    println!("{}", id_sum);
}

struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl From<&str> for Game {
    fn from(game_as_str: &str) -> Self {

        // parse game id
        //        vv
        // Game 50: 9 red; 5 green, 2 blue, 10 red; 5 red, 1 green
        // ^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        // |        line_rounds
        // line_game
        let (line_game, line_rounds) = game_as_str.split_once(": ").unwrap();

        //     v
        // Game 50
        // ^^^^ ^^
        // _    line_id
        let (_, line_id) = line_game.split_once(" ").unwrap();
        let id = line_id.parse().unwrap();

        // extract rounds
        let mut rounds: Vec<Round> = Vec::new();

        //      vv                       vv
        // 9 red; 5 green, 2 blue, 10 red; 5 red, 1 green
        // ^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^
        // |      |                        line_round[n]  ...
        // |      line_round[1]
        // line_round[0]
        for line_round in line_rounds.split("; ") {
            rounds.push(Round::from(line_round));
        }

        Self { id, rounds }
    }
}

impl Game {
    fn possible(&self, bag: &Bag) -> bool {
        // a Game with a certain Bag is possible,
        // iff ALL Rounds are possible with this Bag
        self.rounds.iter()
            .all(|round| round.possible(bag))
    }
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<&str> for Round {
    fn from(line_round: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        //        vv      vv
        // 5 green, 2 blue, 10 red
        // ^^^^^^^  ^^^^^^  ^^^^^^
        // cube[0]  cube[1] cube[n]  ...
        for cube in line_round.split(", ").collect::<Vec<_>>() {

            //  v
            // 5 green
            // ^ ^^^^^
            // | cube_color
            // cube_count
            let (cube_count, cube_color) = cube.split_once(" ").unwrap();

            let cubes_color: Color = cube_color.into();
            let cubes_count = cube_count.parse().unwrap();

            match cubes_color {
                Red => red = cubes_count,
                Green => green = cubes_count,
                Blue => blue = cubes_count
            }
        }

        Self { red, green, blue }
    }
}

impl Round {
    fn possible(&self, bag: &Bag) -> bool {
        // a Round is possible,
        // iff the Round needs less or equal the amount of cubes in the Bag
        self.red <= bag.red
            && self.green <= bag.green
            && self.blue <= bag.blue
    }
}

enum Color { Red, Green, Blue }

impl From<&str> for Color {
    fn from(cubes_color: &str) -> Self {
        match cubes_color {
            "red" => Red,
            "green" => Green,
            "blue" => Blue,
            _ => panic!()
        }
    }
}