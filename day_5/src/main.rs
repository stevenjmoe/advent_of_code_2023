fn main() {
    let part_one_test_input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    part_one(part_one_test_input);
}

fn part_one(input: &str) {
    let mut line_number = 0;
    let seeds: Vec<u32>;

    let mut input_iter = input.lines();
    let s: Vec<_> = input_iter
        .next()
        .unwrap()
        .split(':')
        .map(|c| c.trim())
        .collect();

    seeds = s[1].split(' ').map(|c| c.parse::<u32>().unwrap()).collect();
    println!("{:?}", seeds);

    while let Some(line) = input_iter.next() {
        println!("{}", line);
    }
}
