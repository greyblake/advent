use rayon::prelude::*;

fn main() {
    let input = include_str!("input.txt");

    let result1 = compute_part1(input);
    println!("Part1 Result: {}", result1);

    let result2 = compute_part2(input);
    println!("Part2 Result: {}", result2);
}


fn compute_part1(input: &str) -> u64 {
    let data = parse::parse(input);
    let Data {
        seeds,
        maps,
    } = data;

    let pipeline = MapPipeline { maps };

    let mut lowest_location = u64::MAX;
    for seed in seeds {
        let result = pipeline.convert(seed);
        if result < lowest_location {
            lowest_location = result;
        }
    }

    lowest_location
}

fn compute_part2(input: &str) -> u64 {
    let data = parse::parse(input);
    let Data {
        seeds,
        maps,
    } = data;

    let pipeline = MapPipeline { maps };

    // This calculations takes a long time, so we'll use rayon to parallelize it.
    let mut lowest_locations = seeds.par_chunks(2)
        .map(|pair| {
            let start = pair[0];
            let length = pair[1];

            let mut lowest = u64::MAX;
            for seed in start..(start + length) {
                let location = pipeline.convert(seed);
                if location < lowest {
                    lowest = location;
                }
            }
            lowest
        })
        .collect::<Vec<u64>>();

    lowest_locations.sort();
    lowest_locations[0]
}

fn compute_lowest_location(seeds: Vec<u64>, maps: Vec<Map>) -> u64 {
    let mut current_values = seeds;
    for map in maps {
        current_values = map.convert_many(&current_values);
    }
    current_values.sort();
    current_values[0]
}


#[derive(Debug)]
struct Data {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    full_name: String,
    source_category: String,
    destination_category: String,
    ranges: Vec<MapRange>
}

#[derive(Debug)]
struct MapRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl MapRange {
    fn contains(&self, source: u64) -> bool {
        self.source_start <= source && source < self.source_start + self.length
    }

    fn convert(&self, source: u64) -> u64 {
        let offset = source - self.source_start;
        self.destination_start + offset
    }
}

impl Map {
    fn convert(&self, source: u64) -> u64 {
        for range in &self.ranges {
            if range.contains(source) {
                return range.convert(source);
            }
        }
        source
    }

    fn convert_many(&self, sources: &[u64]) -> Vec<u64> {
        sources.iter().map(|s| self.convert(*s)).collect()
    }
}

struct MapPipeline {
    maps: Vec<Map>,
}

impl MapPipeline {
    fn convert(&self, source: u64) -> u64 {
        let mut current = source;
        for map in &self.maps {
            current = map.convert(current);
        }
        current
    }
}



mod parse {
    use super::*;

    #[derive(Debug)]
    enum Item {
        Seeds(Vec<u64>),
        Map(Map),
    }

    pub fn parse(input: &str) -> Data {
        let mut seeds: Option<Vec<u64>> = None;
        let mut maps: Vec<Map> = Vec::new();

        for raw_item in input.split("\n\n") {
            match parse_item(raw_item) {
                Item::Seeds(new_seeds) => {
                    assert!(seeds.is_none());
                    seeds = Some(new_seeds);
                }
                Item::Map(new_map) => maps.push(new_map),
            }
        }
        let Some(seeds) = seeds else {
            panic!("No seeds defined");
        };

        Data {
            seeds,
            maps,
        }
    }

    fn parse_item(raw_item: &str) -> Item {
        let mut parts = raw_item.trim().split(":");
        let header = parts.next().unwrap().trim();
        let body = parts.next().unwrap().trim();
        assert!(parts.next().is_none());

        if header == "seeds" {
            let seeds = parse_seeds(body);
            Item::Seeds(seeds)
        } else if header.ends_with(" map") {
            let map = parse_map(header, body);
            Item::Map(map)
        } else {
            panic!("Unknown header: {}", header);
        }
    }

    fn parse_seeds(raw_seeds: &str) -> Vec<u64> {
        raw_seeds.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    }

    fn parse_map(header: &str, body: &str) -> Map {
        let mut parts = header.split(" ");
        let from_to = parts.next().unwrap();
        let mut from_to_parts = from_to.split("-to-");

        let from = from_to_parts.next().unwrap();
        let to = from_to_parts.next().unwrap();

        let mut ranges = Vec::new();
        for raw_range in body.split("\n") {
            ranges.push(parse_map_range(raw_range));
        }

        ranges.sort_by_key(|r| r.source_start);

        Map {
            full_name: header.to_string(),
            source_category: from.to_string(),
            destination_category: to.to_string(),
            ranges,
        }
    }

    fn parse_map_range(raw_range: &str) -> MapRange {
        let mut parts = raw_range.split(" ");
        let destination_start = parts.next().unwrap().parse().unwrap();
        let source_start = parts.next().unwrap().parse().unwrap();
        let length = parts.next().unwrap().parse().unwrap();
        assert!(parts.next().is_none());

        MapRange {
            destination_start,
            source_start,
            length,
        }
    }
}





#[test]
fn should_compute_part1() {
    let input =
r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;
    assert_eq!(compute_part1(input), 35);
    assert_eq!(compute_part2(input), 46);
}
