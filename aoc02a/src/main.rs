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
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<&str> for Round {
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
}

fn main() {
    let g = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    dbg!(g);
}
