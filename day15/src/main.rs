use std::vec::Vec;

fn compute_hash(x: &Vec<char>, label_only: bool) -> i64 {
    let mut hash = 0;

    for c in x {
        if label_only && (*c as u8) < 65 {
            break;
        }
        hash += *c as i64;
        hash *= 17;
        hash %= 256;
    }

    hash
}

fn split_at_delimiter(lens: &Vec<char>) -> (Vec<char>, Option<i64>) {
    if lens.iter().any(|x| *x == '=') {
        let position = lens.iter().position(|s| *s == '=').unwrap();
        let number = lens[position + 1..]
            .iter()
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        return (lens[0..position].to_vec(), Some(number));
    } else {
        let position = lens.iter().position(|s| *s == '-').unwrap();
        return (lens[0..position].to_vec(), None);
    }
}

fn find_index(lens: &Vec<char>, lenses: &Vec<(Vec<char>, i64)>) -> Option<usize> {
    let mut index: Option<usize> = None;
    for i in 0..lenses.len() {
        if lenses[i].0 == *lens {
            index = Some(i);
            break;
        }
    }

    index
}

fn main() {
    let input = include_str!("../input.txt")
        .split(',')
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let pt1 = input.iter().map(|x| compute_hash(x, false)).sum::<i64>();
    println!("{}", pt1);

    let mut hashtable: Vec<Vec<(Vec<char>, i64)>> = Vec::new();
    hashtable.resize(255, vec![]);
    for lens in input {
        let hash = compute_hash(&lens, true) as usize;

        let (label, power) = split_at_delimiter(&lens);

        if power.is_some() {
            let index = find_index(&label, &hashtable[hash]);

            if let Some(i) = index {
                hashtable[hash][i].1 = power.unwrap();
            } else {
                hashtable[hash].push((label, power.unwrap()));
            }
        } else {
            let index = find_index(&label, &hashtable[hash]);

            if let Some(i) = index {
                hashtable[hash].remove(i);
            }
        }
    }

    let pt2 = hashtable
        .iter()
        .enumerate()
        .map(|(i, h)| {
            (i as i64 + 1)
                * h.iter()
                    .enumerate()
                    .map(|(j, x)| (j as i64 + 1) * x.1)
                    .sum::<i64>()
        })
        .sum::<i64>();

    println!("{}", pt2);
}
