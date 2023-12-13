use lazy_static::lazy_static;
use regex::{self, Regex};
use std::{collections::HashMap, io};

#[derive(Debug, Clone, Copy)]
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

fn parse_symbol_columns(line: &str) -> Vec<(usize, bool)> {
    SYMBOL_PATTERN
        .find_iter(line)
        .map(|m| (m.start(), m.as_str() == "*"))
        .collect()
}

fn find_gears(row: usize, col: usize, nums: &HashMap<usize, Vec<Num>>) -> Option<(Num, Num)> {
    let adjacent_nums: Vec<&Num> = (row.saturating_sub(1)..(row + 2))
        .map(|num_row| match nums.get(&num_row) {
            None => vec![],
            Some(nums) => nums
                .iter()
                .filter(|num| num.is_adjacent(row, col))
                .collect(),
        })
        .flatten()
        .collect();
    if adjacent_nums.len() == 2 {
        Some((*adjacent_nums[0], *adjacent_nums[1]))
    } else {
        None
    }
}

fn main() {
    let mut nums: Vec<Num> = Vec::new();
    let mut symbols_by_row: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut gear_symbols_by_row: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut row = 0;
    for line in io::stdin().lines() {
        let line = line.expect("Read line");

        nums.append(&mut Num::parse_line(row, &line));

        symbols_by_row.insert(
            row,
            parse_symbol_columns(&line)
                .iter()
                .map(|(col, _)| *col)
                .collect(),
        );

        gear_symbols_by_row.insert(
            row,
            parse_symbol_columns(&line)
                .iter()
                .filter(|(col, is_gear_symbol)| *is_gear_symbol)
                .map(|(col, _)| *col)
                .collect(),
        );

        row += 1;
    }

    // println!("{:?}", nums);
    // println!("{:?}", symbols_by_row);
    // println!("{:?}", gear_symbols_by_row);

    // 531932 is correct

    let mut nums_by_row: HashMap<usize, Vec<Num>> = HashMap::new();
    for num in nums {
        if !nums_by_row.contains_key(&num.row) {
            nums_by_row.insert(num.row, Vec::new());
        }
        nums_by_row.get_mut(&num.row).unwrap().push(num);
    }

    let gears: Option<Vec<(Num, Num)>> = gear_symbols_by_row
        .iter()
        .map(|(row, cols)| {
            cols.iter()
                .map(|col| find_gears(*row, *col, &nums_by_row))
                .filter(|gears| gears.is_some())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    // println!("{:?}", gears);

    let gear_ratios: usize = gears
        .unwrap()
        .iter()
        .map(|(lhs, rhs)| lhs.val * rhs.val)
        .sum();
    println!("{gear_ratios}");
    // 73646890 is correct
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
