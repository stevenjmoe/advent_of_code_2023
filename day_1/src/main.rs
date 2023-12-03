use std::fs;

fn main() {
    let mut nums: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let mut numeric_chars: Vec<char> = Vec::new();

        for c in line.chars() {
            if c.is_numeric() {
                numeric_chars.push(c);
            }
        }

        let mut num = String::from(numeric_chars.first().unwrap().to_string());
        num.push(*numeric_chars.last().unwrap());
        nums.push(num.parse().unwrap());

        sum += num.parse::<u32>().unwrap();
    }

    println!("{sum}");
}
