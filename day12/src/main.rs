use std::collections::HashMap;
use std::vec::Vec;

fn nck(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    let mut k = k;
    if k * 2 > n {
        k = n - k;
    }
    if k == 0 {
        return 1;
    }
    let mut result = n;
    for i in 2..=k {
        result *= n - i + 1;
        result /= i;
    }
    result
}

struct PermutationGenerator {
    n: u64,
    x: u64,
    i: u64,
}

impl PermutationGenerator {
    fn new(size: u64, qmark_hint: u64) -> PermutationGenerator {
        PermutationGenerator {
            n: nck(size, qmark_hint),
            x: (1u64 << qmark_hint as u64) - 1,
            i: 0,
        }
    }

    fn next_combination(a: u64) -> u64 {
        let c = a & (!a + 1);
        let r = a + c;
        (((r ^ a) >> 2) / c) | r
    }
}
impl Iterator for PermutationGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.n {
            return None;
        }
        self.i += 1;

        let old = self.x;
        if self.x != 0 {
            self.x = PermutationGenerator::next_combination(self.x);
        }

        return Some(old);
    }
}

fn count_combinations(spring: &Vec<char>, combination: &Vec<i64>) -> i64 {
    let q_count = spring.iter().filter(|x| **x == '?').count() as u64;

    let hash_count = spring.iter().filter(|x| **x == '#').count() as u64;
    let qmark_hint = combination.iter().sum::<i64>() as u64;
    let perms = PermutationGenerator::new(q_count, qmark_hint - hash_count);

    perms
        .filter(|perm| {
            let mut consecutive_count = 0;
            let mut qmark_count = 0;

            let expected_qs = perm.count_ones();
            let mut bit_count = 0;

            let mut last_index: Option<usize> = None;
            let mut iterator = spring.iter().enumerate().filter_map(|(i, x)| {
                last_index = Some(i);
                if *x == '?' {
                    let old_qmark = qmark_count;
                    qmark_count += 1;

                    if (perm & (1u64 << old_qmark as u64)) == (1u64 << old_qmark as u64) {
                        consecutive_count += 1;
                        bit_count += 1;
                        if i == spring.len() - 1 {
                            return Some(consecutive_count);
                        }
                    } else {
                        if consecutive_count != 0 {
                            let old = consecutive_count;
                            consecutive_count = 0;
                            return Some(old);
                        }
                        consecutive_count = 0;
                    }
                } else if *x == '#' {
                    consecutive_count += 1;
                    if i == spring.len() - 1 {
                        return Some(consecutive_count);
                    }
                } else {
                    if consecutive_count != 0 {
                        let old = consecutive_count;
                        consecutive_count = 0;
                        return Some(old);
                    }
                    consecutive_count = 0;
                }
                None
            });

            let res = combination.iter().all(|a| {
                let b = iterator.next();
                *a == b.unwrap()
            });

            if res == true {
                if last_index.unwrap() == spring.len() - 1 {
                    return true;
                }

                let i = last_index.unwrap();
                if spring[i..].iter().any(|x| *x == '#') {
                    return false;
                }

                if bit_count != expected_qs {
                    return false;
                }

                return true;
            }
            return false;
        })
        .count() as i64
}

fn part1(springs: &Vec<Vec<char>>, combinations: &Vec<Vec<i64>>) -> i64 {
    springs
        .iter()
        .zip(combinations.iter())
        .map(|(spring, combination)| {
            count_combinations(spring, combination)
        })
        .sum::<i64>()
}

fn part2_recursive(
    spring: &[char],
    combination: &[i64],
    mut spring_index: usize,
    mut combination_index: usize,
    cache: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    let mut answer = 0;

    loop {
        if spring_index >= spring.len() {
            if combination_index == combination.len() {
                return 1;
            } else {
                return 0;
            }
        }

        if spring[spring_index] != '.' {
            break;
        }
        spring_index += 1;
    }

    if cache.contains_key(&(spring_index, combination_index)) {
        return *cache.get(&(spring_index, combination_index)).unwrap();
    }

    if spring[spring_index] == '?' {
        let mut lhs = spring.to_vec();
        let mut rhs = lhs.clone();
        lhs[spring_index] = '#';
        rhs[spring_index] = '.';

        answer = part2_recursive(&lhs, combination, spring_index, combination_index, cache)
            + part2_recursive(&rhs, combination, spring_index, combination_index, cache);
    }

    if spring[spring_index] == '#' {
        if combination_index == combination.len() {
            return 0;
        }

        let c = combination[combination_index] as usize;

        if spring_index + c > spring.len() {
            return 0;
        }
        if spring[spring_index..spring_index + c]
            .iter()
            .any(|x| *x == '.')
        {
            return 0;
        }

        if spring_index + c == spring.len() || spring[spring_index + c] != '#' {
            answer = part2_recursive(
                spring,
                combination,
                spring_index + c + 1,
                combination_index + 1,
                cache,
            );
        }
    }

    cache.insert((spring_index, combination_index), answer);
    return answer;
}

fn part2(springs: &Vec<Vec<char>>, combinations: &Vec<Vec<i64>>) -> i64 {
    let mut progress = 0;
    springs
        .iter()
        .zip(combinations.iter())
        .map(|(spring, combination)| {
            let mut cache = HashMap::new();

            let new_combination = combination
                .iter()
                .cycle()
                .take(5 * combination.len())
                .copied()
                .collect::<Vec<i64>>();
            let new_spring = (0..5)
                .flat_map(|i| {
                    let mut temp = spring.clone();
                    if i != 4 {
                        temp.push('?');
                    }
                    temp
                })
                .collect::<Vec<char>>();

            part2_recursive(&new_spring, &new_combination, 0, 0, &mut cache)
        })
        .sum::<i64>()
}

fn main() {
    let input = include_str!("../input.txt");

    let springs = input
        .lines()
        .map(|line| {
            line.split(' ')
                .next()
                .unwrap()
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let combinations = input
        .lines()
        .map(|line| {
            line.split(' ')
                .nth(1)
                .unwrap()
                .split(',')
                .filter_map(|x| x.parse::<i64>().ok())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    println!("{}", part1(&springs, &combinations));
    println!("{}", part2(&springs, &combinations));
}
