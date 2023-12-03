use std::fs::read_to_string;

fn main() {
    let bag = Bag { red: 12, green: 13, blue: 14 };

    let mut id_sum = 0;

    for game_line in read_to_string("src/example").unwrap().lines() {
        let game = Game::from(game_line);
        if game.possible(&bag) {
            id_sum += game.id;
        }
    }

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

        Self { id, rounds }
    }

    fn possible(&self, bag: &Bag) -> bool {
        // a Game with a certain Bag is possible, iff all Rounds are possible with this bag
        self.rounds.iter()
            .all(|r| r.possible(bag))
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

    fn possible(&self, bag: &Bag) -> bool {
        // a round is possible, if the round needs less or equal the amount of cubes in the bag
        self.red <= bag.red
            && self.green <= bag.green
            && self.blue <= bag.blue
    }
}




