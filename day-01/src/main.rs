use std::io;

// 53194 without digitization
// 53706 with digitization is too low

fn main() {
    let r: Result<u32, _> = io::stdin()
        .lines()
        .fold(Ok::<u32, String>(0), |acc: Result<u32, String>, line| {
            Ok(acc? + get_calibration(&line.unwrap())?)
        });
    println!("{:?}", r)
}

fn get_calibration(line: &str) -> Result<u32, String> {
    let digits_from_digits = get_digits(line);
    let digits_from_words = get_digits_from_words(line);
    let mut digits: Vec<(usize, u32)> = digits_from_digits.chain(digits_from_words).collect();
    println!("{:?}", digits);

    if digits.is_empty() {
        return Err(format!("No digits in line '{line}'"));
    }
    digits.sort();

    return Ok(digits.first().unwrap().1 * 10 + digits.last().unwrap().1);
}

/// Returns an iterator over a tuple for each digit consisting of (offset, value).
fn get_digits(line: &str) -> impl Iterator<Item = (usize, u32)> + '_ {
    line.chars()
        .enumerate()
        .filter(|(_i, c)| c.is_digit(10))
        .map(|(i, c)| (i, c.to_digit(10)))
        .map(|(i, o)| (i, o.unwrap()))
}

const DIGIT_WORDS: [&'static str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_digits_from_words(line: &str) -> impl Iterator<Item = (usize, u32)> + '_ {
    DIGIT_WORDS
        .iter()
        .enumerate()
        .map(|(i, el)| {
            line.match_indices(el)
                .map(move |m| (m.0, i as u32))
                .collect::<Vec<(usize, u32)>>()
        })
        .flatten()
}

#[test]
fn test_get_digits_empty() {
    assert_eq!(None, get_digits("").next());
}

#[test]
fn test_get_all_digits() {
    let mut actual: Vec<(usize, u32)> = get_digits("09182736450123456789").collect();
    assert_eq!(20, actual.len());
    actual.sort();

    let expected = vec![0, 9, 1, 8, 2, 7, 3, 6, 4, 5, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for i in 0..expected.len() {
        assert_eq!(expected[i], actual[i].1);
    }
}

#[test]
fn test_get_all_digits_from_words() {
    let mut actual: Vec<(usize, u32)> =
        get_digits_from_words("zero nine one eight two seven three six four five").collect();
    assert_eq!(10, actual.len());
    actual.sort();
    assert_eq!(0, actual[0].1);
    assert_eq!(9, actual[1].1);
    assert_eq!(1, actual[2].1);
    assert_eq!(8, actual[3].1);
    assert_eq!(2, actual[4].1);
    assert_eq!(7, actual[5].1);
    assert_eq!(3, actual[6].1);
    assert_eq!(6, actual[7].1);
    assert_eq!(4, actual[8].1);
    assert_eq!(5, actual[9].1);
}

#[test]
fn test_get_digits_from_overlapping_words() {
    let mut actual: Vec<(usize, u32)> = get_digits_from_words("oneighthreefive").collect();
    assert_eq!(4, actual.len());
    actual.sort();
    assert_eq!((0, 1), actual[0]);
    assert_eq!((2, 8), actual[1]);
    assert_eq!((6, 3), actual[2]);
    assert_eq!((11, 5), actual[3]);
}

#[test]
fn test_calibration_empty_line() {
    assert_eq!(
        get_calibration(""),
        Err(String::from("No digits in line ''"))
    );
}

#[test]
fn test_calibration_no_digits() {
    assert_eq!(
        get_calibration("foobar"),
        Err(String::from("No digits in line 'foobar'"))
    );
}

#[test]
fn test_calibration_single_digit() {
    assert_eq!(get_calibration("foo2bar"), Ok(22));
}

#[test]
fn test_calibration_multiple_digits() {
    assert_eq!(get_calibration("bsevena5z2"), Ok(72));
    assert_eq!(get_calibration("b7a5ztwo"), Ok(72));
}
