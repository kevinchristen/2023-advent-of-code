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

    // part 1
    // println!("num_winners: {num_winners}");
    // if num_winners == 0 {
    //     0
    // } else {
    //     2_usize.pow(num_winners as u32 - 1)
    // }

    // part 2
    num_winners.try_into().unwrap()
}

#[test]
fn test_score_line() {
    assert_eq!(16, score_line("Card   1: 98 16 95 90 53 33 43  7 46 45 | 85 15 78 57 34 10 46 90 33 13  8 54  4 37 25 63 55 41  7 82 69 16 30 76  2"));
}

#[test]
fn test_no_score() {
    assert_eq!(0, score_line("Card  7: 1 | 2"))
}

#[test]
fn test_single_score() {
    assert_eq!(1, score_line("Card  11: 1 | 1"))
}

fn get_copies(i: usize, orig: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    let start = orig.get(i).unwrap();
    for j in (i + 1)..(i + 1 + start.1) {
        let cur = orig.get(j).unwrap();
        res.push(*cur);
        res.append(get_copies(j, orig).as_mut());
    }
    res
}

#[test]
fn test_get_copies() {
    let orig: Vec<(usize, usize)> = vec![(0, 2), (1, 1), (2, 0)];
    let expected: Vec<(usize, usize)> = vec![(1, 1), (2, 0), (2, 0)];
    assert_eq!(expected, get_copies(0, &orig));
}

fn main() {
    // part 1
    // let score = io::stdin()
    //     .lines()
    //     .map(|l| score_line(&l.unwrap()))
    //     .fold(0, |acc, score| acc + score);
    // println!("{score}");

    // part 2
    let originals: Vec<(usize, usize)> = io::stdin()
        .lines()
        .enumerate()
        .map(|(i, l)| (i, score_line(&l.unwrap())))
        .collect();
    println!("{:?}", originals);
    let num_copies1 = originals
        .iter()
        .enumerate()
        .map(|(i, c)| get_copies(i, &originals).len())
        .reduce(|acc, count| acc + count);
    println!("num_copies1 = {:?}", num_copies1);
    // 6227972 is correct
    println!("{}", originals.len() + num_copies1.unwrap());
}
