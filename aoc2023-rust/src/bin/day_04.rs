fn main() {
    let input = include_str!("../../inputs/day04.txt");
    let scores = scores(input);
    let sum = scores
        .iter()
        .map(|score| if *score == 0 { 0 } else { 2u32.pow(score - 1) })
        .sum::<u32>();
    println!("Part one: {}", sum);

    let mut visited = 0;
    let mut queue = (0..scores.len()).collect::<Vec<usize>>();
    while let Some(index) = queue.pop() {
        visited += 1;
        for win in 0..scores[index] {
            queue.push(index + win as usize + 1);
        }
    }
    println!("Part two: {}", visited);
}

fn scores(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            let (winning, have) = line.split_once(':').unwrap().1.split_once('|').unwrap();
            let have = have.split_ascii_whitespace().collect::<Vec<&str>>();
            winning
                .split_ascii_whitespace()
                .filter(|i| have.contains(i))
                .count() as u32
        })
        .collect::<Vec<u32>>()
}
