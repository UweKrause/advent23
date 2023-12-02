use std::fs::read_to_string;

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn from(s: &str) -> Self {
        let mut id = 0;
        let mut rvec: Vec<Round> = Vec::new();

        // parse game id
        let mut gvec: Vec<&str> = s.split(":").collect();
        let gidvec: Vec<&str> = gvec[0].split_whitespace().collect();
        id = gidvec[1].parse().unwrap();

        // extract rounds
        let v0: Vec<&str> = gvec[1].split(";").collect();
        for round in v0 {
            rvec.push(Round::from(round));
        }

        Self {
            id: id,
            rounds: rvec,
        }
    }

    fn possible(&self, bag: &Bag) -> bool {
        // a Game is possible, iff all Rounds are possible
        let mut possible = true;
        for round in &self.rounds {
            possible = possible && round.possible(&bag);
        }
        possible

        // ToDo:
        // I want to write this as iterator
        // something like this:
        // &self.rounds.iter().all(r => r.possible(bag))
        // how?
    }
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn from(s: &str) -> Self {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        let v: Vec<&str> = s.split(", ").collect();


        for x in v {
            let v2: Vec<_> = x.split_whitespace().collect();
            match v2[1] {
                "red" => red = v2[0].parse::<u32>().unwrap(),
                "green" => green = v2[0].parse::<u32>().unwrap(),
                "blue" => blue = v2[0].parse::<u32>().unwrap(),
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

struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let bag = Bag { red: 12, green: 13, blue: 14 };

    for g in read_to_string("src/example").unwrap().lines() {
        let g = Game::from(g);
        println!("{:?} {}", g, g.possible(&bag));
    }
}
