use std::io;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUM_PATTERN: Regex = Regex::new(r"\d+").unwrap();
}

fn score_line(line: &str) -> usize {
    let mut i = line.split("|").into_iter();
    let (winners, yours) = (i.next().unwrap().trim(), i.next().unwrap().trim());
    let winners: Vec<usize> = NUM_PATTERN
        .find_iter(winners)
        .map(|m| usize::from_str_radix(m.as_str(), 10).expect("Scores are numbers"))
        .skip(1) // first line is the card #
        .collect();
    // println!("{:?}", winners);
    let yours: Vec<usize> = NUM_PATTERN
        .find_iter(yours)
        .map(|m| usize::from_str_radix(m.as_str(), 10).expect("Scores are numbers"))
        .collect();
    // println!("{:?}", yours);
    let num_winners = yours.into_iter().filter(|y| winners.contains(y)).count() as u32;
    // println!("num_winners: {num_winners}");
    if num_winners == 0 {
        0
    } else {
        2_usize.pow(num_winners as u32 - 1)
    }
}

#[test]
fn test_score_line() {
    assert_eq!(16 /*16 90 33 7 46 */, score_line("Card   1: 98 16 95 90 53 33 43  7 46 45 | 85 15 78 57 34 10 46 90 33 13  8 54  4 37 25 63 55 41  7 82 69 16 30 76  2"));
}

#[test]
fn test_no_score() {
    assert_eq!(0, score_line("Card  7: 1 | 2"))
}

#[test]
fn test_single_score() {
    assert_eq!(1, score_line("Card  11: 1 | 1"))
}

fn main() {
    let score = io::stdin()
        .lines()
        .map(|l| score_line(&l.unwrap()))
        .fold(0, |acc, score| acc + score);
    println!("{score}");
}
