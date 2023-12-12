use std::collections::HashMap;
use std::io;

#[derive(Debug, PartialEq, Clone, Copy)]
struct CubeCounts {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeCounts {
    fn parse_counts(s: &str) -> CubeCounts {
        let mut count: HashMap<&str, usize> = HashMap::new();
        count.insert("red", 0);
        count.insert("green", 0);
        count.insert("blue", 0);

        let color_counts = s.trim().split(",");
        for color in color_counts {
            let c = color.trim().split_once(" ").expect("Split color and count");
            count.insert(
                c.1.trim(),
                usize::from_str_radix(c.0.trim(), 10)
                    .expect(&format!("Color count must be a number: {}", c.1.trim())),
            );
        }

        Self {
            red: count["red"],
            green: count["green"],
            blue: count["blue"],
        }
    }

    fn power(self: &CubeCounts) -> usize {
        let power = self.red * self.green * self.blue;
        println!("{:?}: {}", self, power);
        power
    }
}

#[derive(Debug)]
struct Game {
    name: String,
    bag_counts: CubeCounts,
    sample_counts: Vec<CubeCounts>,
}

impl Game {
    fn is_possible(self: &Game) -> bool {
        !self
            .sample_counts
            .iter()
            .find(|s| {
                s.red > self.bag_counts.red
                    || s.green > self.bag_counts.green
                    || s.blue > self.bag_counts.blue
            })
            .is_some()
    }

    fn parse_game(line: &str) -> Game {
        let mut i = line.split(":");
        let name = i
            .next()
            .expect("Line must start with game: '{line}'")
            .to_string();
        let samples: Vec<CubeCounts> = i
            .next()
            .expect("Game must contain samples")
            .split(";")
            .map(|s| CubeCounts::parse_counts(s.trim()))
            .collect();

        Self {
            name,
            bag_counts: CubeCounts {
                red: 12,
                green: 13,
                blue: 14,
            },
            sample_counts: samples,
        }
    }

    fn get_game_id(self: &Game) -> usize {
        usize::from_str_radix(
            &self.name[self.name.rfind(" ").expect("Space in game name") + 1..],
            10,
        )
        .expect("Parse id from game name")
    }

    fn smallest_bag(self: &Game) -> CubeCounts {
        let mut acc = CubeCounts {
            red: 0,
            green: 0,
            blue: 0,
        };
        let smallest = self.sample_counts.iter().fold(acc, |lhs, rhs| {
            acc.red = rhs.red.max(lhs.red);
            acc.green = rhs.green.max(lhs.green);
            acc.blue = rhs.blue.max(lhs.blue);
            return acc;
        });
        smallest
    }
}

fn main() {
    // let games = io::stdin()
    //     .lines()
    //     .map(|l| Game::parse_game(&l.expect("Parse game")))
    //     .filter(|g| g.is_possible());
    // let sum = games.fold(0, |acc, i| acc + i.get_game_id());
    // println!("sum is {sum}");

    let sum_of_powers = io::stdin()
        .lines()
        .map(|l| {
            Game::parse_game(&l.expect("Parse game"))
                .smallest_bag()
                .power()
        })
        .fold(0, |acc, p| acc + p);
    println!("sum of powers: {sum_of_powers}");
}

#[test]
fn test_parse_line() {
    let game = Game::parse_game("Game 1: 2 green, 12 blue; 6 red, 6 blue; 8 blue, 5 green, 5 red");
    println!("{:?}", game);

    assert_eq!(3, game.sample_counts.len());
    assert_eq!("Game 1", game.name);
    assert_eq!(
        CubeCounts {
            red: 0,
            green: 2,
            blue: 12,
        },
        game.sample_counts[0]
    );
    assert_eq!(
        CubeCounts {
            red: 6,
            green: 0,
            blue: 6,
        },
        game.sample_counts[1]
    );
    assert_eq!(
        CubeCounts {
            red: 5,
            green: 5,
            blue: 8,
        },
        game.sample_counts[2]
    );
}

#[test]
fn test_power() {
    assert_eq!(
        48,
        CubeCounts {
            red: 4,
            green: 2,
            blue: 6
        }
        .power()
    );
}

#[test]
fn test_smallest_bag() {
    assert_eq!(
        CubeCounts {
            red: 4,
            green: 2,
            blue: 6,
        },
        Game::parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").smallest_bag()
    );
}
