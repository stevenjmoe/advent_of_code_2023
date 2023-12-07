use regex::Regex;
use std::fs;

struct Max {
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

fn main() {
    let mut day_one_result: u32 = 0;
    let max = &Max {
        max_red: 12,
        max_green: 13,
        max_blue: 14,
    };

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        day_one_result += match part_one(line, max) {
            Some(x) => x,
            None => 0,
        }
    }

    println!("{day_one_result}");

    let test_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let mut r: u32 = 0;

    for line in test_string.lines() {
        r += match part_one(line, max) {
            Some(x) => x,
            None => 0,
        }
    }

    assert_eq!(8, r);

    assert_eq!(
        0,
        match part_one(
            "Game 3: 2 green, 4 blue, 13 red; 15 blue, 9 red, 3 green; 3 red, 18 blue, 3 green; 6 red, 4 green, 2 blue; 6 blue, 13 red",
            max,
        ) {
            Some(x) => x,
            None => 0,
        }
    );
}

fn part_one(input: &str, max: &Max) -> Option<u32> {
    let reg = Regex::new(r"\d+").unwrap();

    let game = input.split_once(|c: char| c == ':').unwrap();
    let game_id: u32 = reg.find(game.0).unwrap().as_str().parse().unwrap();
    let game_sets = game.1;

    let sets = game_sets.split(";");
    let mut valid_game = true;

    for set in sets {
        let s = set.split(",");

        for colour in s {
            if colour.to_lowercase().contains("red") {
                let n: u32 = reg.find(colour).unwrap().as_str().parse().unwrap();
                if n > max.max_red {
                    valid_game = false;
                }
            }
            if colour.to_lowercase().contains("green") {
                let n: u32 = reg.find(colour).unwrap().as_str().parse().unwrap();
                if n > max.max_green {
                    valid_game = false;
                }
            }

            if colour.to_lowercase().contains("blue") {
                let n: u32 = reg.find(colour).unwrap().as_str().parse().unwrap();
                if n > max.max_blue {
                    valid_game = false;
                }
            }
        }
    }

    if valid_game {
        Some(game_id)
    } else {
        None
    }
}
