use std::vec::Vec;

// /*
// ???

// ...
// ..#
// .#.
// #..
// ##.
// .##
// #.#
// ###
// */
fn permute(count: i64) -> Vec<Vec<char>> {
    if count == 1 {
        return vec![vec!['#'], vec!['.']];
    }

    permute(count - 1)
        .iter()
        .flat_map(|p| {
            let mut p0 = p.clone();
            let mut p1 = p.clone();

            p0.push('#');
            p1.push('.');
            [p0, p1]
        })
        .collect::<Vec<Vec<char>>>()
}

fn factorial(n: u64) -> u64 {
    (2..=n).product()
}

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

struct PermutationGenerator
{
    n: u64,
    x: u64,
    i: u64
}

impl PermutationGenerator {
    fn new(size: u64, qmark_hint: u64) -> PermutationGenerator {
        PermutationGenerator { n: nck(size, qmark_hint), x: (1u64 << qmark_hint as u64) - 1, i: 0}
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


fn generate_permutations(input: &Vec<char>) -> Vec<Vec<char>> {
    let qmark_count = input.iter().filter(|c| **c == '?').count();
    permute(qmark_count as i64)
}

fn part1(springs: &Vec<Vec<char>>, combinations: &Vec<Vec<i64>>) -> i64 {
    springs
        .iter()
        .zip(combinations.iter())
        .map(|(spring, combination)| {
            let q_count = spring.iter().filter(|x| **x == '?').count() as u64;

            let hash_count = spring.iter().filter(|x| **x == '#').count() as u64;
            let qmark_hint = combination.iter().sum::<i64>() as u64;
            let perms = PermutationGenerator::new(q_count, qmark_hint - hash_count);

            perms
                .filter(|perm| {

                    let mut consecutive_count = 0;
                    let mut qmark_count = 0;

                    // println!("Permutation: {}", perm);
                    // for qq in 0..q_count {
                    //     if (perm & (1u64 << qq as u64)) == (1u64 << qq as u64) {
                    //         print!("{}", '#');
                    //     }
                    //     else {
                    //         print!("{}", '.');
                    //     }
                    // }
                    // println!();

                    let expected_qs = perm.count_ones();
                    let mut bit_count = 0;

                    let mut last_index : Option<usize> = None;
                    let mut iterator = spring.iter().enumerate().filter_map(|(i, x)| {
                        last_index = Some(i);
                        if *x == '?' {
                            let old_qmark = qmark_count;
                            qmark_count += 1;
                            
                            if (perm & (1u64 << old_qmark as u64)) == (1u64 << old_qmark as u64) {
                                consecutive_count += 1;
                                bit_count += 1;
                                // print!("{}", '#');
                                if i == spring.len() - 1{
                                    return Some(consecutive_count);
                                }
                            }
                            else {
                                // print!("{}", '.');
                                if consecutive_count != 0 {
                                    let old = consecutive_count;
                                    consecutive_count = 0;
                                    return Some(old);
                                }
                                consecutive_count = 0;
                            }
                        }
                        else if *x == '#' {
                            // print!("{}", '#');
                            consecutive_count += 1;
                            if i == spring.len() - 1 {
                                return Some(consecutive_count);
                            }
                        }
                        else {
                            // print!("{}", '.');
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
                        // println!("{} vs {}", *a, b.unwrap());
                        *a == b.unwrap()
                    });

                    // println!("Matched: {}", res);
                    if res == true {
                        if last_index.unwrap() == spring.len() - 1 {
                            // println!("returning true");
                            return true;
                        }

                        let i = last_index.unwrap();
                        if spring[i..].iter().any(|x| *x == '#') {
                            // println!("returning false on hashes");
                            return false;
                        }

                        if bit_count != expected_qs {
                            // println!("returning false on not enough questionmarks: {} {}", bit_count, expected_qs);
                            return false;
                        }
                        
                        // println!("returning true");
                        return true;
                    }
                    // println!("returning false");
                    return false;
                })
                .count() as i64
        })
        .sum::<i64>()
}

fn part2(springs: &Vec<Vec<char>>, combinations: &Vec<Vec<i64>>) -> i64 {
    springs
        .iter()
        .zip(combinations.iter())
        .map(|(spring, combination)| {
            let new_combination = combination.iter().cycle().take(5*combination.len()).copied().collect::<Vec<i64>>();
            let new_spring = (0..5).flat_map(|i| {
                let mut temp = spring.clone();
                if i != 4 {
                    temp.push('?');
                }
                temp
            }).collect::<Vec<char>>();

            println!("{:?}", new_combination);
            println!("{:?}", new_spring.iter().collect::<String>());

            let q_count = new_spring.iter().filter(|x| **x == '?').count() as u64;

            let hash_count = new_spring.iter().filter(|x| **x == '#').count() as u64;
            let qmark_hint = new_combination.iter().sum::<i64>() as u64;

            let perms = PermutationGenerator::new(q_count, qmark_hint - hash_count);

            println!("{} {}", q_count, qmark_hint - hash_count);

            let total = nck(q_count, qmark_hint - hash_count);

            let tmp = perms
                .enumerate().filter(|(pp, perm)| {

                    if (pp % 10000000) == 0 {
                        println!("{}%", (100.0 * (*pp as f64)) / (total as f64));
                    }
                    let mut consecutive_count = 0;
                    let mut qmark_count = 0;

                    // println!("Permutation: {}", perm);
                    // for qq in 0..q_count {
                    //     if (perm & (1u64 << qq as u64)) == (1u64 << qq as u64) {
                    //         print!("{}", '#');
                    //     }
                    //     else {
                    //         print!("{}", '.');
                    //     }
                    // }
                    // println!();

                    let expected_qs = perm.count_ones();
                    let mut bit_count = 0;

                    let mut last_index : Option<usize> = None;
                    let mut iterator = new_spring.iter().enumerate().filter_map(|(i, x)| {
                        last_index = Some(i);
                        if *x == '?' {
                            let old_qmark = qmark_count;
                            qmark_count += 1;
                            
                            if (perm & (1u64 << old_qmark as u64)) == (1u64 << old_qmark as u64) {
                                consecutive_count += 1;
                                bit_count += 1;
                                // print!("{}", '#');
                                if i == new_spring.len() - 1{
                                    return Some(consecutive_count);
                                }
                            }
                            else {
                                // print!("{}", '.');
                                if consecutive_count != 0 {
                                    let old = consecutive_count;
                                    consecutive_count = 0;
                                    return Some(old);
                                }
                                consecutive_count = 0;
                            }
                        }
                        else if *x == '#' {
                            // print!("{}", '#');
                            consecutive_count += 1;
                            if i == new_spring.len() - 1 {
                                return Some(consecutive_count);
                            }
                        }
                        else {
                            // print!("{}", '.');
                            if consecutive_count != 0 {
                                let old = consecutive_count;
                                consecutive_count = 0;
                                return Some(old);
                            }
                            consecutive_count = 0;
                        }
                        None
                    });

                    let res = new_combination.iter().all(|a| {
                        let b = iterator.next();
                        // println!("{} vs {}", *a, b.unwrap());
                        *a == b.unwrap()
                    });

                    // println!("Matched: {}", res);
                    if res == true {
                        if last_index.unwrap() == new_spring.len() - 1 {
                            // println!("returning true");
                            return true;
                        }

                        let i = last_index.unwrap();
                        if new_spring[i..].iter().any(|x| *x == '#') {
                            // println!("returning false on hashes");
                            return false;
                        }

                        if bit_count != expected_qs {
                            // println!("returning false on not enough questionmarks: {} {}", bit_count, expected_qs);
                            return false;
                        }
                        
                        // println!("returning true");
                        return true;
                    }
                    // println!("returning false");
                    return false;
                })
                .count() as i64;
                println!("{}", tmp);
                tmp
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

    // println!("{}", part1(&springs, &combinations));
    println!("{}", part2(&springs, &combinations));
}
