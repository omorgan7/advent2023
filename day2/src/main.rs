use std::vec::Vec;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Draw {
    Red(i64),
    Blue(i64),
    Green(i64),
}

#[derive(Debug)]
struct Game {
    id: i64,
    draws: Vec<Vec<Draw>>,
}

fn to_inner(x: Option<Draw>) -> i64 {
    if x.is_none() {
        return 0;
    }
    match x.unwrap() {
        Draw::Red(c) => c,
        Draw::Blue(c) => c,
        Draw::Green(c) => c,
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let games = input
        .lines()
        .map(|line| {
            let mut colon_split = line.split(':');
            let id = colon_split.next().unwrap()[5..].parse::<i64>().unwrap();

            let draws = colon_split.next().unwrap()[1..]
                .split(';')
                .map(|subgame| {
                    subgame
                        .split(',')
                        .map(|draw| {
                            let mut moves = draw.split(' ');
                            let count_string = {
                                let tmp = moves.next().unwrap();
                                if tmp.is_empty() {
                                    moves.next().unwrap()
                                } else {
                                    tmp
                                }
                            };

                            let count = count_string.parse::<i64>().unwrap();
                            
                            match moves.next().unwrap() {
                                "blue" => Draw::Blue(count),
                                "red" => Draw::Red(count),
                                "green" => Draw::Green(count),
                                _ => panic!(),
                            }
                        })
                        .collect::<Vec<Draw>>()
                })
                .collect::<Vec<Vec<Draw>>>();
            Game { id, draws }
        })
        .collect::<Vec<Game>>();

    let pt1 = games.iter().fold(0, |sum, game| {
        if game.draws.iter().any(|subgame| {
            subgame.iter().any(|draw| match draw {
                Draw::Red(c) => *c > 12,
                Draw::Blue(c) => *c > 14,
                Draw::Green(c) => *c > 13,
            })
        }) {
            sum
        } else {
            sum + game.id
        }
    });
    println!("{}", pt1);
    let pt2 = games.iter().fold(0, |sum, game| {
        let min = game.draws.iter().fold(
            (None, None, None),
            |mut curr: (Option<Draw>, Option<Draw>, Option<Draw>), subgame| {
                let mut red: Option<Draw> = None;
                let mut blue: Option<Draw> = None;
                let mut green: Option<Draw> = None;
                subgame.iter().for_each(|draw| match draw {
                    Draw::Red(_) => red = Some(*draw),
                    Draw::Blue(_) => blue = Some(*draw),
                    Draw::Green(_) => green = Some(*draw),
                });

                let filter_max = |c: &mut Option<Draw>, d| {
                    if c.is_none() {
                        *c = d
                    } else {
                        *c = if to_inner(*c) < to_inner(d) {
                            d
                        } else {
                            Some(c.unwrap())
                        }
                    }
                };

                filter_max(&mut curr.0, red);
                filter_max(&mut curr.1, blue);
                filter_max(&mut curr.2, green);

                curr
            },
        );
        sum + to_inner(min.0) * to_inner(min.1) * to_inner(min.2)
    });
    println!("{}", pt2);
}
