use std::collections::HashMap;
use std::vec::Vec;

fn part1(instructions: &Vec<usize>, nodes: &HashMap<String, [String; 2]>) -> i64 {
    let mut pc = 0;
    let mut total = 0;
    let mut current = "AAA".to_string();

    while current != "ZZZ" {
        let new = &nodes.get(&current).unwrap()[instructions[pc]];
        total += 1;
        pc += 1;
        pc %= instructions.len();
        current = new.to_string();
    }

    total
}

struct InstructionMachine<'b> {
    instructions: &'b Vec<usize>,
    pc: usize,
}

impl<'b> InstructionMachine<'b> {
    fn get_next<'a>(
        &mut self,
        nodes: &'a HashMap<String, [String; 2]>,
        current: &String,
    ) -> &'a String {
        let new = &nodes.get(current).unwrap()[self.instructions[self.pc]];
        self.pc += 1;
        self.pc %= self.instructions.len();

        new
    }
}

fn lcm(x: &[i64]) -> i64 {
    let factors = x
        .iter()
        .map(|c| prime_factorisation(*c))
        .collect::<Vec<Vec<(i64, i64)>>>();

    let mut prod = 1;
    for i in 0..factors[0].len() {
        let max_power = factors.iter().max_by(|a, b| a[i].1.cmp(&b[i].1)).unwrap();
        prod *= factors[0][i].0.pow(max_power[i].1 as u32);
    }

    prod
}

fn prime_factorisation(x: i64) -> Vec<(i64, i64)> {
    let _limit = (x as f64).sqrt().ceil() as i64;

    // laziness
    let primes = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277,
    ];

    let mut factors = Vec::<(i64, i64)>::new();

    for p in primes {
        let mut i = 0;
        let mut tmp = x;

        while tmp % p == 0 && tmp != 0 {
            i += 1;
            tmp /= p;
        }
        factors.push((p, i));
    }

    factors
}

fn part2(instructions: &Vec<usize>, nodes: &HashMap<String, [String; 2]>) -> i64 {
    let _pc = 0;
    let _total = 0;
    let currents = nodes
        .keys()
        .filter(|node| node.chars().nth(2).unwrap() == 'A')
        .collect::<Vec<&String>>();
    let detected_cycles = currents
        .iter()
        .map(|c| {
            // tortoise and hare cycle detection algorithm
            let mut hare_instructions = InstructionMachine {
                instructions,
                pc: 0,
            };
            let mut tortoise_instructions = InstructionMachine {
                instructions,
                pc: 0,
            };

            let mut tortoise = tortoise_instructions.get_next(nodes, c);
            let mut hare = hare_instructions.get_next(nodes, c);
            hare = hare_instructions.get_next(nodes, hare);

            let mut seen_z = false;
            while !(tortoise == hare && seen_z) {
                tortoise = tortoise_instructions.get_next(nodes, tortoise);
                if tortoise.chars().nth(2).unwrap() == 'Z' {
                    seen_z = true;
                }
                hare = hare_instructions.get_next(nodes, hare);
                hare = hare_instructions.get_next(nodes, hare);
            }

            let mut first_repetition = 0;
            tortoise_instructions.pc = 0;
            tortoise = c;
            seen_z = false;

            while !(tortoise == hare && seen_z) {
                tortoise = tortoise_instructions.get_next(nodes, tortoise);
                if tortoise.chars().nth(2).unwrap() == 'Z' {
                    seen_z = true;
                }
                hare = hare_instructions.get_next(nodes, hare);
                first_repetition += 1;
            }

            let mut cycle_length = 1;
            hare = hare_instructions.get_next(nodes, tortoise);
            seen_z = false;
            while !(tortoise == hare && seen_z) {
                hare = hare_instructions.get_next(nodes, hare);
                if hare.chars().nth(2).unwrap() == 'Z' {
                    seen_z = true;
                }
                cycle_length += 1;
            }

            cycle_length
        })
        .collect::<Vec<i64>>();

    lcm(&detected_cycles)
}

fn main() {
    let input = include_str!("../input.txt");

    let instructions = input
        .lines().next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<usize>>();

    let nodes = input
        .lines()
        .skip(2)
        .map(|line| {
            let node_name = line[0..3].to_string();

            let children = [line[7..10].to_string(), line[12..15].to_string()];

            (node_name, children)
        })
        .collect::<HashMap<String, [String; 2]>>();

    println!("{}", part1(&instructions, &nodes));
    println!("{}", part2(&instructions, &nodes));
}
