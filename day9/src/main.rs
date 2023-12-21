fn pt1_recurse(history: &Vec<i64>, difference: &mut i64) {
    if history.iter().all(|x| *x == history[0]) {
        *difference = history[0];
        return;
    }

    let mut differences = Vec::new();

    for i in 1..history.len() {
        differences.push(history[i] - history[i - 1]);
    }

    let mut recurse_diff = 0;
    pt1_recurse(&differences, &mut recurse_diff);

    *difference = history.last().unwrap() + recurse_diff;
}

fn pt2_recurse(history: &Vec<i64>, difference: &mut i64) {
    if history.iter().all(|x| *x == history[0]) {
        *difference = history[0];
        return;
    }

    let mut differences = Vec::new();

    for i in 1..history.len() {
        differences.push(history[i] - history[i - 1]);
    }

    let mut recurse_diff = 0;
    pt2_recurse(&differences, &mut recurse_diff);

    *difference = history.first().unwrap() - recurse_diff;
}

fn part1(input: &Vec<Vec<i64>>) -> i64 {
    input
        .iter()
        .map(|h| {
            let mut difference = 0;
            pt1_recurse(h, &mut difference);

            difference
        })
        .sum::<i64>()
}

fn part2(input: &Vec<Vec<i64>>) -> i64 {
    input
        .iter()
        .map(|h| {
            let mut difference = 0;
            pt2_recurse(h, &mut difference);

            difference
        })
        .sum::<i64>()
}

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
