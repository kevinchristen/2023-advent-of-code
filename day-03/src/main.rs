use lazy_static::lazy_static;
use regex::{self, Regex};
use std::{arch::is_aarch64_feature_detected, collections::HashMap, io};

#[derive(Debug)]
struct Num {
    row: usize,
    start_col: usize,
    end_col: usize,
    val: usize,
}

lazy_static! {
    static ref NUM_PATTERN: Regex = Regex::new(r"\d+").unwrap();
    static ref SYMBOL_PATTERN: Regex = Regex::new(r"[^\d.]").unwrap();
}

impl Num {
    fn parse_line(row: usize, line: &str) -> Vec<Num> {
        NUM_PATTERN
            .find_iter(line)
            .map(|m| Num {
                row,
                start_col: m.start(),
                end_col: m.end(),
                val: usize::from_str_radix(m.as_str(), 10).unwrap(),
            })
            .collect()
    }

    fn is_adjacent(self: &Self, row: usize, col: usize) -> bool {
        row >= self.row.saturating_sub(1)
            && row <= self.row + 1
            && col >= self.start_col.saturating_sub(1)
            && col <= self.end_col // end column of match is the column of the next char after the match
    }
}

fn parse_symbol_columns(line: &str) -> Vec<usize> {
    SYMBOL_PATTERN
        .find_iter(line)
        .map(|m| {
            println!("{:?}", m);
            m.start()
        })
        .collect()
}

fn main() {
    let mut nums: Vec<Num> = Vec::new();
    let mut symbols_by_row: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut row = 0;
    for line in io::stdin().lines() {
        let line = line.expect("Read line");
        nums.append(&mut Num::parse_line(row, &line));

        symbols_by_row.insert(row, parse_symbol_columns(&line));
        row += 1;
    }

    println!("{:?}", nums);
    println!("{:?}", symbols_by_row);

    let nums_by_symbols = nums
        .into_iter()
        .filter(|n| {
            (n.row > 0
                && symbols_by_row
                    .get(&(n.row - 1))
                    .unwrap()
                    .iter()
                    .any(|c| n.is_adjacent(n.row - 1, *c)))
                || symbols_by_row
                    .get(&n.row)
                    .unwrap()
                    .iter()
                    .any(|c| n.is_adjacent(n.row, *c))
                || (symbols_by_row.contains_key(&(n.row + 1))
                    && symbols_by_row
                        .get(&(n.row + 1))
                        .unwrap()
                        .iter()
                        .any(|c| n.is_adjacent(n.row + 1, *c)))
        })
        .map(|n| n.val)
        .reduce(|acc, n| {
            println!("adding {acc} + {n}");
            acc + n
        });
    println!("{:?}", nums_by_symbols);
    ()

    // 538792 too high
    // 531932 is correct
}

#[test]
fn test_is_adjacent_basic() {
    let n = Num {
        row: 2,
        start_col: 2,
        end_col: 4,
        val: 1,
    };

    // Row too high
    for i in 0..6 {
        assert!(!n.is_adjacent(0, i));
    }

    // Row before
    assert!(!n.is_adjacent(1, 0));
    for i in 1..5 {
        assert!(n.is_adjacent(1, i));
    }
    assert!(!n.is_adjacent(1, 5));

    // Same row
    assert!(!n.is_adjacent(2, 0));
    assert!(n.is_adjacent(2, 1));
    assert!(n.is_adjacent(2, 4));
    assert!(!n.is_adjacent(2, 5));

    // Row after
    assert!(!n.is_adjacent(3, 0));
    for i in 1..5 {
        assert!(n.is_adjacent(3, i));
    }
    assert!(!n.is_adjacent(3, 5));

    // Row too low
    for i in 0..6 {
        assert!(!n.is_adjacent(4, i));
    }
}
