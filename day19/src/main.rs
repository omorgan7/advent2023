use std::collections::HashMap;
use std::ops::Range;

fn part_to_index(c: char) -> usize {
    match c {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => panic!(),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ComparisonType {
    Less,
    Greater,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Compare {
    t: ComparisonType,
    part: char,
    value: i64,
    if_true: SubInstruction
}

impl Compare {
    fn call(&self, x: &Range<i64>) -> [Range<i64>; 2] {
        match self.t {
            ComparisonType::Less => {
                [x.start..self.value-1, self.value..x.end]
            }
            ComparisonType::Greater => {
                [self.value+1..x.end, x.start..self.value]
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Ranges
{
    x: Range<i64>,
    m: Range<i64>,
    a: Range<i64>,
    s: Range<i64>
}

impl Ranges
{
    fn new() -> Ranges {
        Ranges { x: 1..4000, m: 1..4000, a: 1..4000, s: 1..4000 }
    }

    fn get_range(&self, c: char) -> &Range<i64> {
        match c
        {
            'x' => &self.x,
            'm' => &self.m,
            'a' => &self.a,
            's' => &self.s,
            _ => panic!()
        }
    }

    fn get_range_mut(&mut self, c: char) -> &mut Range<i64> {
        match c
        {
            'x' => &mut self.x,
            'm' => &mut self.m,
            'a' => &mut self.a,
            's' => &mut self.s,
            _ => panic!()
        }
    }

    fn accumulate(&self) -> i64 {
        ((self.x.start..=self.x.end).count() *
        (self.m.start..=self.m.end).count() *
        (self.a.start..=self.a.end).count() *
        (self.s.start..=self.s.end).count()) as i64
    }
}

#[test]
fn test_accumulate() -> Result<(), String> {

    let mut range = Ranges::new();
    range.x = 2000..4000;

    let mut count = 0;

    for x in range.x.clone() {
        count += 1;
    }

    assert_eq!(2000, count);
    assert_eq!(128000000000000, range.accumulate());
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum SubInstruction {
    Accept,
    Reject,
    Call(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Cmp(Compare),
    Accept,
    Reject,
    Call(String),
}

#[derive(Clone, Debug)]
struct Function {
    instructions: Vec<Instruction>,
}

fn pt2(functions: &HashMap<String, Function>, instruction_ptr: &String, mut current_range : Ranges, mut accumulated_ranges: Vec<Ranges>) -> Vec<Ranges>
{
    let function = functions.get(instruction_ptr).unwrap();

    for i in function.instructions.iter() {
        match i {
            Instruction::Cmp(comparer) => {
                let subranges = comparer.call(current_range.get_range(comparer.part));
                match &comparer.if_true {
                    SubInstruction::Accept => {
                        let mut subrange = current_range.clone();
                        *subrange.get_range_mut(comparer.part) = subranges[0].clone();
                        accumulated_ranges.push(subrange)
                    },
                    SubInstruction::Reject => {},
                    SubInstruction::Call(next) => {
                        let mut subrange = current_range.clone();
                        *subrange.get_range_mut(comparer.part) = subranges[0].clone();
                        let tmp = pt2(functions, &next, subrange, accumulated_ranges);
                        accumulated_ranges = tmp;
                    }
                }
                *current_range.get_range_mut(comparer.part) = subranges[1].clone();
            },
            Instruction::Accept => {
                accumulated_ranges.push(current_range.clone());
                return accumulated_ranges;
            },
            Instruction::Reject => {
                return accumulated_ranges;
            }
            Instruction::Call(next) => {
                let tmp = pt2(functions, &next, current_range.clone(), accumulated_ranges);
                accumulated_ranges = tmp;
            }
        }
    }
    return accumulated_ranges;
}

fn main() {
    let input = include_str!("../input.txt");

    let parts = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| {
            let subline = &line[1..line.len() - 1];

            let mut it = subline.split(',').map(|s| s[2..].parse::<i64>().unwrap());

            [
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
            ]
        })
        .collect::<Vec<[i64; 4]>>();

    let instructions = {
        let mut tmp = HashMap::new();
        for line in input.split("\n\n").next().unwrap().lines() {
            let mut it = line.split('{');
            let key = it.next().unwrap();

            let value = it.next().unwrap();

            tmp.insert(key.to_string(), value[0..value.len() - 1].to_string());
        }

        tmp
    };

    let pt1 = parts
        .iter()
        .filter(|p| {
            let mut next = "in".to_string();

            loop {
                let instruction = instructions.get(&next).unwrap();

                let chars = instruction.chars().collect::<Vec<char>>();
                let mut i = 0;

                loop {
                    if i == chars.len() {
                        break;
                    }

                    let next_c = chars[i];
                    if (i + 1) < chars.len() && (chars[i + 1] == '<' || chars[i + 1] == '>') {
                        let pn = p[part_to_index(next_c)];
                        let cmp = match chars[i + 1] {
                            '<' => std::cmp::Ordering::Less,
                            '>' => std::cmp::Ordering::Greater,
                            _ => panic!(),
                        };

                        i += 1;

                        let mut numbers = Vec::<char>::new();

                        while chars[i + 1] != ':' {
                            numbers.push(chars[i + 1]);
                            i += 1;
                        }
                        i += 1;

                        let n = numbers
                            .iter()
                            .rev()
                            .fold((0, 1), |(acc, mul), x| {
                                (acc + mul * x.to_digit(10).unwrap() as i64, mul * 10)
                            })
                            .0;

                        let mut next_instruction = Vec::<char>::new();
                        while chars[i + 1] != ',' {
                            next_instruction.push(chars[i + 1]);
                            i += 1;
                        }
                        i += 1;

                        let next_instruction_s = next_instruction.iter().collect::<String>();

                        if pn.cmp(&n) == cmp {
                            if next_instruction_s == "A" {
                                return true;
                            }
                            if next_instruction_s == "R" {
                                return false;
                            }

                            next = next_instruction_s;
                            break;
                        }
                        i += 1;
                    } else {
                        next = chars[i..].iter().collect::<String>();
                        if next == "R" {
                            return false;
                        }
                        if next == "A" {
                            return true;
                        }
                        break;
                    }
                }
            }
        })
        .flatten()
        .sum::<i64>();

    println!("{:?}", pt1);

    let functions = instructions
        .iter()
        .map(|(k, v)| {
            let mut function = Function {
                instructions: Vec::new(),
            };
            let mut i = 0;
            let chars = v.chars().collect::<Vec<char>>();

            loop {
                if i == chars.len() {
                    break;
                }

                let next_c = chars[i];

                if (i + 1) < chars.len() && (chars[i + 1] == '<' || chars[i + 1] == '>') {
                    i += 1;
                    let mut numbers = Vec::<char>::new();

                    let cmp_symbol = chars[i];

                    while chars[i + 1] != ':' {
                        numbers.push(chars[i + 1]);
                        i += 1;
                    }
                    i += 1;

                    let n = numbers
                        .iter()
                        .rev()
                        .fold((0, 1), |(acc, mul), x| {
                            (acc + mul * x.to_digit(10).unwrap() as i64, mul * 10)
                        })
                        .0;

                    let mut next_instruction = Vec::<char>::new();
                    while chars[i + 1] != ',' {
                        next_instruction.push(chars[i + 1]);
                        i += 1;
                    }
                    i += 1;
                    let next_instruction_s = next_instruction.iter().collect::<String>();
                    let sub_instruction = if next_instruction_s == "A" {
                        SubInstruction::Accept
                    } else if next_instruction_s == "R" {
                        SubInstruction::Reject
                    } else {
                        SubInstruction::Call(next_instruction_s)
                    };

                    function.instructions.push(Instruction::Cmp(Compare {
                        t: if cmp_symbol == '<' {
                            ComparisonType::Less
                        } else {
                            ComparisonType::Greater
                        },
                        part: next_c,
                        value: n,
                        if_true: sub_instruction
                    }));
                    i += 1;
                } else {
                    let next = chars[i..].iter().collect::<String>();
                    function.instructions.push(if next == "A" {
                        Instruction::Accept
                    } else if next == "R" {
                        Instruction::Reject
                    } else {
                        Instruction::Call(next)
                    });
                    break;
                }
            }

            (k.clone(), function)
        })
        .collect::<HashMap<String, Function>>();

    let accumulated_ranges = pt2(&functions, &"in".to_string(), Ranges::new(), vec![]);
    println!("{:?}", accumulated_ranges.iter().map(|r| r.accumulate()).sum::<i64>());
}
