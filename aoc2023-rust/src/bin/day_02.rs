fn main() {
    let input = include_str!("../../inputs/day02.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let mut split = line.split([' ', ':', ',', ';']).skip(3);
            while let Some(token) = split.next() {
                let Ok(number) = token.parse::<u32>() else {
                    continue;
                };

                match split.next().unwrap() {
                    "red" if number > 12 => return 0,
                    "green" if number > 13 => return 0,
                    "blue" if number > 14 => return 0,
                    _ => (),
                }
            }
            id as u32 + 1
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (mut red, mut green, mut blue) = (0, 0, 0);
            let mut split = line.split([' ', ':', ',', ';']).skip(3);
            while let Some(token) = split.next() {
                let Ok(number) = token.parse::<u32>() else {
                    continue;
                };

                match split.next().unwrap() {
                    "red" if number > red => red = number,
                    "green" if number > green => green = number,
                    "blue" if number > blue => blue = number,
                    _ => (),
                }
            }
            red * green * blue
        })
        .sum()
}
