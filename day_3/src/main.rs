use regex::Regex;
use std::fs;

#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
}

struct NumberPosition {
    positions: Vec<Position>,
    value: String,
}

fn main() {
    let part_one_result = part_one(fs::read_to_string("input.txt").unwrap());
    println!("{part_one_result}");

    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let test_result = part_one(input.to_string());
    assert_eq!(4361, test_result);
}

fn part_one(input: String) -> u32 {
    let symbol_reg = Regex::new(r"[^\.\d]").unwrap();
    let number_reg = Regex::new(r"[\d]").unwrap();
    let mut y: i32 = 0;
    let mut number_positions: Vec<NumberPosition> = vec![];
    let mut valid_positions: Vec<Position> = vec![];

    for line in input.lines() {
        let mut x: i32 = 0;

        let mut current_char: char = '.';
        let mut current_combined_number: String = String::from("");
        let mut positions: Vec<Position> = vec![];

        let mut iter = line.chars().peekable();
        while let Some(char) = iter.next() {
            if let Some(l) = symbol_reg.find(&char.to_string()) {
                match l.as_str().parse() {
                    Ok(r) => {
                        valid_positions.push(Position { x: x + 1, y });
                        valid_positions.push(Position { x: x - 1, y });
                        valid_positions.push(Position { x: x - 1, y: y + 1 });
                        valid_positions.push(Position { x, y: y + 1 });
                        valid_positions.push(Position { x: x + 1, y: y + 1 });
                        valid_positions.push(Position { x: x - 1, y: y - 1 });
                        valid_positions.push(Position { x, y: y - 1 });
                        valid_positions.push(Position { x: x + 1, y: y - 1 });

                        r
                    }
                    Err(_) => '.',
                };
            }

            if let Some(l) = number_reg.find(&char.to_string()) {
                match l.as_str().parse() {
                    Ok(r) => {
                        current_char = r;
                        r
                    }

                    Err(_) => '.',
                };
            } else {
                current_char = '.';
            }

            if current_char != '.' {
                current_combined_number.push(current_char);
                positions.push(Position { x, y });
            }

            if (current_char == '.' && !current_combined_number.is_empty()) || iter.peek().is_none()
            {
                number_positions.push(NumberPosition {
                    positions: positions.clone(),
                    value: current_combined_number.to_owned(),
                });

                current_combined_number = String::from("");
                positions = vec![];
            }

            x += 1;
        }

        y += 1;
    }

    let mut result: u32 = 0;
    let mut count: u32 = 0;

    for number_position in number_positions {
        let mut valid = false;

        for position in &number_position.positions {
            if valid {
                break;
            }

            for valid_position in &valid_positions {
                if position.x == valid_position.x && position.y == valid_position.y {
                    valid = true;
                    break;
                }
            }

            if valid {
                if count < 50 {
                    let val = &number_position.value;
                    count += 1;
                }

                result += number_position.value.parse::<u32>().unwrap();
                break;
            }
        }
    }
    println!("{result}");
    result
}
