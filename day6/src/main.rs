use std::vec::Vec;

fn main() {
    let input = include_str!("../input.txt");

    let times: Vec<i64> = input.lines().nth(0).unwrap()[5..]
        .split(' ')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();
    let best_distances: Vec<i64> = input.lines().nth(1).unwrap()[9..]
        .split(' ')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    let pt1 = times
        .iter()
        .zip(best_distances.iter())
        .fold(1, |prod, (time, distance)| {
            let mut better_count = 0;
            for i in 1..(*time - 1) {
                if i * (time - i) > *distance {
                    better_count += 1;
                }
            }
            prod * better_count
        });

    println!("{}", pt1);

    let time = input.lines().nth(0).unwrap()[5..]
        .split(' ')
        .filter(|s| *s != " ")
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let distance = input.lines().nth(1).unwrap()[9..]
        .split(' ')
        .filter(|s| *s != " ")
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    let mut better_count = 0;
    for i in 1..(time - 1) {
        if i * (time - i) > distance {
            better_count += 1;
        }
    }

    println!("{}", better_count);
}
