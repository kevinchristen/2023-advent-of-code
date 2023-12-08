use std::io;

fn main() {
    let r: Result<u32, _> = io::stdin()
        .lines()
        .fold(Ok::<u32, String>(0), |acc: Result<u32, String>, line| {
            Ok(acc? + get_calibration(&line.unwrap())?)
        });
    println!("{:?}", r)
}

fn get_calibration(line: &str) -> Result<u32, String> {
    let mut digits = line.chars().filter(|c| c.is_digit(10));
    let first = match digits.next() {
        Some(digit) => digit.to_digit(10),
        None => return Err(format!("No digits in line '{line}'")),
    };
    let first = first.unwrap(); // safe because of filtering by is_digit()
    let last = match digits.last() {
        Some(digit) => digit.to_digit(10).unwrap(), // safe because of filtering by is_digit()
        None => first,
    };
    return Ok(first * 10 + last);
}

#[test]
fn test_empty_line() {
    assert_eq!(
        get_calibration(""),
        Err(String::from("No digits in line ''"))
    );
}

#[test]
fn test_no_digits() {
    assert_eq!(
        get_calibration("foobar"),
        Err(String::from("No digits in line 'foobar'"))
    );
}

#[test]
fn test_single_digit() {
    assert_eq!(get_calibration("foo2bar"), Ok(22));
}

#[test]
fn test_multiple_digit() {
    assert_eq!(get_calibration("b7a5z2"), Ok(72));
}
