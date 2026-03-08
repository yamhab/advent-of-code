const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = include_str!("../../inputs/day01.txt");
    println!("Part one: {}", calibration_sum(input));

    let mut input = input.to_string();
    for (digit, word) in DIGIT_WORDS.into_iter().enumerate() {
        input = input.replace(word, &format!("{}{}{0}", word, digit + 1));
    }
    println!("Part two: {}", calibration_sum(&input));
}

fn calibration_sum(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|r#char| r#char.to_digit(10))
                .collect::<Vec<u32>>();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}
