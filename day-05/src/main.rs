use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, Read},
    ops::Range,
    str::Lines,
};

lazy_static! {
    static ref MAP_LINE_PARSE_PATTERN: Regex = Regex::new(r"\s*(\d+)\s+(\d+)\s+(\d+)\s*").unwrap();
}

struct Mapper {
    mappers: HashMap<Range<usize>, Box<dyn Fn(usize) -> usize>>,
}

impl Mapper {
    fn new() -> Mapper {
        Mapper {
            mappers: HashMap::new(),
        }
    }

    fn insert(self: &mut Self, k: Range<usize>, v: Box<dyn Fn(usize) -> usize>) {
        self.mappers.insert(k, v);
    }

    fn parse_and_insert(self: &mut Self, line: &str) {
        let caps = MAP_LINE_PARSE_PATTERN.captures(line).unwrap();
        let dest_start = usize::from_str_radix(caps.get(1).expect("Capture dest").as_str(), 10)
            .expect("Parse dest");
        let src_start = usize::from_str_radix(caps.get(2).expect("Capture src").as_str(), 10)
            .expect("Parse src");
        let len = usize::from_str_radix(caps.get(3).expect("Capture length").as_str(), 10)
            .expect("Parse length");

        self.insert(
            Range {
                start: src_start,
                end: src_start + len,
            },
            Box::new(move |i| i + dest_start - src_start),
        );
    }

    fn parse_map<'a>(lines: &mut Lines<'a>) -> Option<(&'a str, Mapper)> {
        let name = lines.next()?;

        let mut mapper = Mapper::new();
        for l in lines {
            if l.is_empty() {
                break;
            }
            mapper.parse_and_insert(l);
        }
        // skip suffix " map:"
        Some((&name[..name.len() - 5], mapper))
    }

    fn map(self: &Self, k: usize) -> usize {
        let f = self.mappers.iter().filter(|(k1, _)| k1.contains(&k)).nth(0);
        match f {
            Some((_, b)) => b(k),
            None => k,
        }
    }
}

#[test]
fn test_mapper() {
    let mut m = Mapper::new();
    m.insert(0..3, Box::new(|i| i + 1));
    m.insert(3..6, Box::new(|i| i * 2));
    m.insert(6..10, Box::new(|i| i - 1));
    for i in 0..3 {
        assert_eq!(i + 1, m.map(i));
    }
    for i in 3..6 {
        assert_eq!(i * 2, m.map(i));
    }
    for i in 6..10 {
        assert_eq!(i - 1, m.map(i));
    }
}

#[test]
fn test_mapper_line_parsing() {
    let mut m = Mapper::new();
    m.parse_and_insert("10 0 2");
    for i in 0..2 {
        assert_eq!(i + 10, m.map(i));
    }
    assert_eq!(2, m.map(2));
}

#[test]
fn test_mapper_out_of_range() {
    let m = Mapper::new();
    assert_eq!(7, m.map(7));
}

#[test]
fn test_parse_map() {
    let s = "fertilizer-to-water map:
2261570026 3454758517 88568015
1802864872 1796719466 196521844
1029796290 924285105 94936250

water-to-light map:
12536522 803922375 381092756
1034093555 2274122448 375903462
";

    let mut it = s.lines();
    let (mut name, mut mapper) = Mapper::parse_map(&mut it).unwrap();
    assert_eq!("fertilizer-to-water", name);
    assert_eq!(3, mapper.mappers.len());
    assert_eq!(1029796290, mapper.map(924285105));

    (name, mapper) = Mapper::parse_map(&mut it).unwrap();
    assert_eq!("water-to-light", name);
    assert_eq!(2, mapper.mappers.len());
    assert_eq!(12536522, mapper.map(803922375));
}

fn parse_seeds(line: &str) -> Vec<usize> {
    // Skip prolog "seeds: "
    line[7..]
        .split(" ")
        .into_iter()
        .map(|s| usize::from_str_radix(s, 10).expect("Parse a seed"))
        .collect()
}

#[test]
fn test_parse_seeds() {
    assert_eq!(
        vec![91926764, 235794528, 3279509610],
        parse_seeds("seeds: 91926764 235794528 3279509610")
    );
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Read stdin");
    let mut lines = input.lines();
    let seeds = parse_seeds(lines.next().expect("Parse seeds"));

    // consume empty line after "seeds: ...""
    lines.next();

    let mut mappers: Vec<(&str, Mapper)> = Vec::new();

    while let Some(m) = Mapper::parse_map(&mut lines) {
        mappers.push(m);
    }

    let mut seeds_to_locations: Vec<(usize, usize)> = seeds
        .iter()
        .map(|s| (*s, mappers.iter().fold(*s, |acc, el| el.1.map(acc))))
        .collect();

    seeds_to_locations.sort_by_cached_key(|i| i.1);

    println!("seeds to locations: {:?}", seeds_to_locations);
    // 551761867 is correct (seeds_to_locations.get(0).1)
}
