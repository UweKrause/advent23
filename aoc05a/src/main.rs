use std::collections::{BTreeMap};
use std::fs::read_to_string;

fn main() {
    let greenhouse: Greenhouse = Greenhouse::from(read_to_string("src/input").unwrap().trim());

    let mut lowest: u64 = u64::MAX;

    for seed in greenhouse.seeds {
        let soil = greenhouse.maps[&0].get_destination(seed);
        let fertilizer = greenhouse.maps[&1].get_destination(soil);
        let water = greenhouse.maps[&2].get_destination(fertilizer);
        let light = greenhouse.maps[&3].get_destination(water);
        let temperature = greenhouse.maps[&4].get_destination(light);
        let humidity = greenhouse.maps[&5].get_destination(temperature);
        let location = greenhouse.maps[&6].get_destination(humidity);
        // dbg!(seed, soil, fertilizer,water, light, temperature, humidity, location);

        lowest = lowest.min(location);
    }

    println!("{}", lowest);
}

#[derive(Debug)]
struct Greenhouse {
    seeds: Vec<u64>,
    maps: BTreeMap<usize, Map>, // using BtreeMap to preserve (and nicely show) order
}

impl Greenhouse {
    fn from(s: &str) -> Greenhouse {
        //                   vvvv
        // seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n [...]
        // ^^^^^^^^^^^^^^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        // split_seeds           split_maps
        let (split_seeds, split_maps) = s.split_once("\n\n").unwrap();

        //      vv
        // seeds: 79 14 55 13
        // ^^^^^  ^^^^^^^^^^^
        // _      split_seeds_numbers
        let (_, split_seeds_numbers) = split_seeds.split_once(": ").unwrap();

        //   v  v  v
        // 79 14 55 13
        // ^^ ^^ ^^ ^^
        // seeds
        let seeds: Vec<u64> = split_seeds_numbers
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        //                                     vvvv
        // seed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n [...]
        // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        // split_maps_maps[0]                      split_maps_maps[1]                  ...split_maps_maps[n]
        let split_maps_maps: Vec<&str> = split_maps.split("\n\n").collect();

        let mut maps: BTreeMap<usize, Map> = BTreeMap::new();
        for (i, split_maps_map) in split_maps_maps.into_iter().enumerate() {
            maps.insert(i, Map::from(split_maps_map));
        }

        Greenhouse { seeds, maps }
    }
}


#[derive(Debug)]
struct Map {
    _name: String,
    ranges: Vec<Range>,
}

impl Map {
    fn from(s: &str) -> Map {
        //                 vvv
        // seed-to-soil map:\n50 98 2\n52 50 48
        // ^^^^^^^^^^^^^^^^   ^^^^^^^^^^^^^^^^^
        let (split_name, split_ranges) = s.split_once(":\n").unwrap();

        //             v
        // seed-to-soil map
        // ^^^^^^^^^^^^ ^^^
        // name         _
        let (_name, _) = split_name.split_once(" ").unwrap();
        let _name = _name.to_string();

        //        vv
        // 50 98 2\n52 50 48
        // ^^^^^^^  ^^^^^^^^
        let mut ranges: Vec<Range> = Vec::new();
        for range in split_ranges.split("\n") {
            ranges.push(Range::from(range))
        }

        Self { _name, ranges }
    }

    fn get_destination(&self, source: u64) -> u64 {
        // Check which range applies (if any)
        match self.find_range_id(source) {
            // If a Range can handle the mapping, let the Range figure out the destination
            Some(id) => self.ranges[id].get_destination(source),
            // If no Range can handle the mapping, output is input
            None => source
        }
    }

    fn find_range_id(&self, source: u64) -> Option<usize> {
        self.ranges.iter()
            .enumerate()
            .find(|(_, x)| x.applies(source))
            .map(|(id, _)| id)
    }
}


#[derive(Debug)]
struct Range {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl Range {
    fn from(s: &str) -> Self {
        // 50 98 2
        // ^^ ^^ ^
        // |  |  length
        // |  source_start
        // destination_start
        let mut split = s.split_whitespace();
        let destination_start: u64 = split.next().unwrap().parse().unwrap();
        let source_start: u64 = split.next().unwrap().parse().unwrap();
        let length: u64 = split.next().unwrap().parse().unwrap();
        // not happy...

        Range { destination_start, source_start, length }
    }

    fn applies(&self, source: u64) -> bool {
        // A Range applies iff the source is within the range
        // A Range is defined by its start and end (inclusive)

        let start = self.source_start;
        let end = self.source_start + self.length;

        start <= source && source <= end
    }

    fn get_destination(&self, source: u64) -> u64 {
        source - self.source_start + self.destination_start
    }
}
