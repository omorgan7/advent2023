use std::collections::HashSet;
use std::vec::Vec;

fn part1(input: &Vec<i64>) -> i64 {
    input.iter().fold(0, |prev, count| {
        if *count == 0 {
            prev
        } else {
            prev + (1 << (*count - 1))
        }
    })
}

fn part2(input: &Vec<i64>) -> i64 {
    let card_count = input.len();
    let mut multiplier = Vec::<i64>::new();
    multiplier.resize(card_count, 1);
    input
        .iter()
        .enumerate()
        .fold((0, multiplier), |(prev, mut multiplier), (i, count)| {
            for j in 0..*count {
                multiplier[i + 1 + j as usize] += multiplier[i];
            }

            (prev + multiplier[i] * *count, multiplier)
        })
        .0
        + card_count as i64
}

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let it = line
                .split(": ")
                .skip(1)
                .map(|lline| {
                    lline
                        .split(" | ")
                        .map(|numbers| {
                            numbers.split(' ')
                                .filter_map(|x| x.parse::<i64>().ok())
                                .collect::<HashSet<i64>>()
                        })
                        .collect::<Vec<HashSet<i64>>>()
                })
                .next()
                .unwrap();
                
            it[0].intersection(&it[1]).count() as i64
        })
        .collect::<Vec<i64>>();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
