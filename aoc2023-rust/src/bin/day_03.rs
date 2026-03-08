use std::collections::hash_map::HashMap;

fn main() {
    let input = include_str!("../../inputs/day03.txt");
    let lines = lines(input);
    let number_map = number_map(&lines);

    let mut part1 = Vec::new();
    let mut part2 = Vec::new();
    for (y, l) in lines.into_iter().enumerate() {
        for (x, c) in l
            .into_iter()
            .enumerate()
            .filter(|(_, c)| !c.is_ascii_digit() && *c != '.')
        {
            let adjacent = adjacent(&number_map, x, y);
            for number in adjacent.iter() {
                part1.push(*number);
            }
            if c == '*' && adjacent.len() == 2 {
                part2.push(adjacent.into_iter().product::<u32>());
            }
        }
    }

    println!("Part one: {}", part1.into_iter().sum::<u32>());
    println!("Part two: {}", part2.into_iter().sum::<u32>());
}

fn lines(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| {
            let mut line = line.chars().collect::<Vec<char>>();
            line.push('.');
            line
        })
        .collect::<Vec<Vec<char>>>()
}

fn number_map(lines: &[Vec<char>]) -> HashMap<(usize, usize), u32> {
    let mut numbers = HashMap::new();
    for (y, l) in lines.iter().enumerate() {
        let mut current = None;
        let mut positions = Vec::new();
        for (x, c) in l.iter().enumerate() {
            let Some(d) = c.to_digit(10) else {
                let Some(n) = current else { continue };
                for position in positions.iter() {
                    numbers.insert((*position, y), n);
                }
                current = None;
                positions.clear();
                continue;
            };

            current = match current {
                Some(n) => Some(n * 10 + d),
                None => Some(d),
            };
            positions.push(x);
        }
    }
    numbers
}

fn adjacent(number_map: &HashMap<(usize, usize), u32>, x: usize, y: usize) -> Vec<u32> {
    let mut adjacent = Vec::new();
    for n in [
        number_map.get(&(x + 1, y + 1)),
        number_map.get(&(x + 1, y - 1)),
        number_map.get(&(x + 1, y)),
        number_map.get(&(x - 1, y + 1)),
        number_map.get(&(x - 1, y - 1)),
        number_map.get(&(x - 1, y)),
        number_map.get(&(x, y + 1)),
        number_map.get(&(x, y - 1)),
    ] {
        if !adjacent.contains(&n) {
            adjacent.push(n);
        }
    }
    adjacent
        .into_iter()
        .flatten()
        .copied()
        .collect::<Vec<u32>>()
}
