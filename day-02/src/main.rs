use std::collections::HashMap;
use std::io;

#[derive(Debug, PartialEq)]
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
}

fn main() {
    let games = io::stdin()
        .lines()
        .map(|l| Game::parse_game(&l.expect("Parse game")))
        .filter(|g| g.is_possible());
    let sum = games.fold(0, |acc, i| acc + i.get_game_id());
    println!("sum is {sum}");
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
