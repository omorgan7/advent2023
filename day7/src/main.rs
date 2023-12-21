use std::cmp::Ordering;
use std::collections::HashMap;
use std::vec::Vec;

fn compare_equal_pt1(aa: &[char], bb: &[char]) -> Ordering {
    for (a, b) in aa.iter().zip(bb.iter()) {
        if a.is_ascii_digit() && b.is_ascii_digit() {
            let cmp = a.to_digit(10).unwrap().cmp(&b.to_digit(10).unwrap());
            if cmp != Ordering::Equal {
                return cmp;
            }
        } else if a.is_ascii_digit() {
            return Ordering::Less;
        } else if b.is_ascii_digit() {
            return Ordering::Greater;
        } else {
            let cmp = compare_picture_cards_pt1(*a, *b);
            if cmp == Ordering::Equal {
                continue;
            }
            return cmp;
        }
    }
    panic!();
}

fn compare_equal_pt2(aa: &[char], bb: &[char]) -> Ordering {
    let map_digit = |c: char| {
        let d = c.to_digit(10);
        if d.is_some() {
            d.unwrap()
        } else {
            match c {
                'J' => 1,
                'T' => 10,
                'Q' => 11,
                'K' => 12,
                'A' => 13,
                _ => panic!(),
            }
        }
    };

    for (a, b) in aa.iter().zip(bb.iter()) {
        let cmp = map_digit(*a).cmp(&map_digit(*b));
        if cmp != Ordering::Equal {
            return cmp;
        }
    }
    panic!();
}

fn compare_conditional_pt1(
    a: bool,
    b: bool,
    a_first: &[char],
    b_first: &[char],
) -> Option<Ordering> {
    if a && b {
        Some(compare_equal_pt1(a_first, b_first))
    } else if a {
        Some(Ordering::Greater)
    } else if b {
        Some(Ordering::Less)
    } else {
        None
    }
}

fn compare_conditional_pt2(
    a: bool,
    b: bool,
    a_first: &[char],
    b_first: &[char],
) -> Option<Ordering> {
    if a && b {
        Some(compare_equal_pt2(a_first, b_first))
    } else if a {
        Some(Ordering::Greater)
    } else if b {
        Some(Ordering::Less)
    } else {
        None
    }
}

fn compare_picture_cards_pt1(a: char, b: char) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }
    match a {
        'A' => Ordering::Greater,
        'K' => {
            if b == 'A' {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
        'Q' => {
            if b == 'A' || b == 'K' {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
        'J' => {
            if b == 'A' || b == 'K' || b == 'Q' {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
        'T' => Ordering::Less,
        _ => panic!(),
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut plays: Vec<(Vec<char>, i64)> = input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            (
                split.next().unwrap().chars().collect(),
                split.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect();

    plays.sort_by(|a, b| {
        let mut a_count = HashMap::<char, i64>::new();
        let mut b_count = HashMap::<char, i64>::new();

        a.0.iter()
            .for_each(|c| *a_count.entry(*c).or_insert(0) += 1);
        b.0.iter()
            .for_each(|c| *b_count.entry(*c).or_insert(0) += 1);

        if *a_count.values().max().unwrap() == 4
            || *a_count.values().max().unwrap() == 5
            || *b_count.values().max().unwrap() == 4
            || *b_count.values().max().unwrap() == 5
        {
            let cmp = a_count
                .values()
                .max()
                .unwrap()
                .cmp(b_count.values().max().unwrap());
            if cmp != Ordering::Equal {
                return cmp;
            } else {
                return compare_equal_pt1(&a.0, &b.0);
            }
        }

        let a_is_fh = a_count.len() == 2 && *a_count.values().max().unwrap() == 3;
        let b_is_fh = b_count.len() == 2 && *b_count.values().max().unwrap() == 3;
        if let Some(cmp) = compare_conditional_pt1(a_is_fh, b_is_fh, &a.0, &b.0) {
            return cmp;
        }

        let a_is_three = a_count.len() == 3 && *a_count.values().max().unwrap() == 3;
        let b_is_three = b_count.len() == 3 && *b_count.values().max().unwrap() == 3;
        if let Some(cmp) = compare_conditional_pt1(a_is_three, b_is_three, &a.0, &b.0) {
            return cmp;
        }

        let a_is_two_pair = a_count.len() == 3 && *a_count.values().max().unwrap() == 2;
        let b_is_two_pair = b_count.len() == 3 && *b_count.values().max().unwrap() == 2;
        if let Some(cmp) = compare_conditional_pt1(a_is_two_pair, b_is_two_pair, &a.0, &b.0) {
            return cmp;
        }

        let a_is_pair = a_count.len() == 4 && *a_count.values().max().unwrap() == 2;
        let b_is_pair = b_count.len() == 4 && *b_count.values().max().unwrap() == 2;
        if let Some(cmp) = compare_conditional_pt1(a_is_pair, b_is_pair, &a.0, &b.0) {
            return cmp;
        }

        compare_equal_pt1(&a.0, &b.0)
    });

    println!(
        "{}",
        plays
            .iter()
            .enumerate()
            .map(|(i, p)| { (i as i64 + 1) * p.1 })
            .sum::<i64>()
    );

    plays.sort_by(|a, b| {
        let mut a_count = HashMap::<char, i64>::new();
        let mut b_count = HashMap::<char, i64>::new();

        a.0.iter()
            .for_each(|c| *a_count.entry(*c).or_insert(0) += 1);
        b.0.iter()
            .for_each(|c| *b_count.entry(*c).or_insert(0) += 1);

        let get_max_with_jokers = |counts: &HashMap<char, i64>| {
            let jokers = *counts.get(&'J').unwrap_or(&0);

            jokers
                + *counts
                    .iter()
                    .filter(|(k, _)| **k != 'J')
                    .map(|(_, v)| v)
                    .max()
                    .unwrap_or(&0)
        };

        let get_next_best_no_jokers = |counts: &HashMap<char, i64>| {
            let max = counts
                .iter()
                .fold(
                    ('X', -1),
                    |best, (k, v)| {
                        if best.1 < *v {
                            (*k, *v)
                        } else {
                            best
                        }
                    },
                )
                .0;

            *counts
                .iter()
                .filter(|(k, _)| **k != 'J' && **k != max)
                .map(|(_, v)| v)
                .max()
                .unwrap()
        };

        if let Some(cmp) = compare_conditional_pt2(
            get_max_with_jokers(&a_count) == 5,
            get_max_with_jokers(&b_count) == 5,
            &a.0,
            &b.0,
        ) {
            return cmp;
        }

        if let Some(cmp) = compare_conditional_pt2(
            get_max_with_jokers(&a_count) == 4,
            get_max_with_jokers(&b_count) == 4,
            &a.0,
            &b.0,
        ) {
            return cmp;
        }

        let a_is_fh = get_max_with_jokers(&a_count) == 3 && get_next_best_no_jokers(&a_count) == 2;
        let b_is_fh = get_max_with_jokers(&b_count) == 3 && get_next_best_no_jokers(&b_count) == 2;
        if let Some(cmp) = compare_conditional_pt2(a_is_fh, b_is_fh, &a.0, &b.0) {
            return cmp;
        }

        if let Some(cmp) = compare_conditional_pt2(
            get_max_with_jokers(&a_count) == 3,
            get_max_with_jokers(&b_count) == 3,
            &a.0,
            &b.0,
        ) {
            return cmp;
        }

        let a_is_two_pair =
            get_max_with_jokers(&a_count) == 2 && get_next_best_no_jokers(&a_count) == 2;
        let b_is_two_pair =
            get_max_with_jokers(&b_count) == 2 && get_next_best_no_jokers(&b_count) == 2;
        if let Some(cmp) = compare_conditional_pt2(a_is_two_pair, b_is_two_pair, &a.0, &b.0) {
            return cmp;
        }

        if let Some(cmp) = compare_conditional_pt2(
            get_max_with_jokers(&a_count) == 2,
            get_max_with_jokers(&b_count) == 2,
            &a.0,
            &b.0,
        ) {
            return cmp;
        }

        compare_equal_pt2(&a.0, &b.0)
    });

    println!(
        "{}",
        plays
            .iter()
            .enumerate()
            .map(|(i, p)| { (i as i64 + 1) * p.1 })
            .sum::<i64>()
    );
}
