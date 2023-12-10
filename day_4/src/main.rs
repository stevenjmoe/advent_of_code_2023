use std::fs;

fn main() {
    let part_one_test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let part_one_test_result = part_one(&part_one_test_input.to_string());
    assert_eq!(13, part_one_test_result);

    let part_one_result = part_one(&fs::read_to_string("input.txt").unwrap());
    println!("{}", part_one_result);
}

fn part_one(input: &String) -> u32 {
    let mut result: u32 = 0;

    for line in input.lines() {
        let mut card_result: u32 = 0;

        let card: Vec<_> = line
            .split(|c| c == ':' || c == '|')
            .map(|c| c.trim())
            .collect();

        let card_id_index = card[0].find(|c: char| c.is_digit(10)).unwrap();

        // card id
        let _ = card[0]
            .get(card_id_index..)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let winning_numbers: Vec<_> = card[1].split(" ").filter(|c| !c.is_empty()).collect();
        let numbers: Vec<_> = card[2].split(" ").filter(|c| !c.is_empty()).collect();

        let mut count: u32 = 0;

        for number in &numbers {
            if winning_numbers.contains(number) {
                count += 1;
                if count == 1 {
                    card_result += 1;
                } else {
                    card_result = card_result * 2;
                }
            }
        }

        result += card_result;
    }

    result
}
