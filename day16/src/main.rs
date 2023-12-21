use std::vec::Vec;
use std::collections::HashSet;

struct Grid {
    grid: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(height: usize, width: usize, grid: Vec::<char>) -> Grid {
        Grid {
            grid,
            width,
            height,
        }
    }

    fn at(&self, (x, y): (i64, i64)) -> Option<char> {
        if x < 0 || (x as usize) >= self.width {
            return None;
        }
        if y < 0 || (y as usize) >= self.height {
            return None;
        }

        Some(self.grid[(x as usize) + self.width * (y as usize)])
    }
}

fn render_grid(visited: &HashSet::<((i64, i64), (i64, i64))>, grid: &Grid) {

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for y in 0..grid.height as i64 {
        for x in 0..grid.width as i64 {
            let mut found = false;
            for d in directions {
                if visited.contains(&(d, (x, y))) {
                    found = true;
                    if d.0 == 1 {
                        print!("{}", ">")
                    }
                    else if d.0 == -1 {
                        print!("{}", "<");
                    }
                    else if d.1 == 1 {
                        print!("{}", "v");
                    }
                    else {
                        print!("{}", "^");
                    }
                    break;
                }
            }
            if found == false {
                print!("{}", grid.at((x, y)).unwrap());
            }
        }
        println!();
    }
    println!();
}

fn part1(grid: &Grid, starting_direction: (i64, i64), starting_position: (i64, i64)) -> i64 {

    let mut stack = Vec::new();
    stack.push((starting_direction, starting_position));

    let mut visited = HashSet::new();

    while !stack.is_empty() {
        // render_grid(&visited, &grid);
        let (direction, position) = stack.pop().unwrap();

        if visited.contains(&(direction, position)) {
            continue;
        }

        let m = grid.at(position);

        if m.is_none() {
            continue;
        }


        visited.insert((direction, position));

        match m.unwrap() {
            '.' => {
                stack.push((direction, (position.0 + direction.0, position.1 + direction.1)));
            },
            '/' => {
                let new_direction = 
                    if direction.0 == 1 {
                        (0, -1)
                    } else if direction.0 == -1 {
                        (0, 1)
                    } else if direction.1 == 1 {
                        (-1, 0)
                    } else {
                        (1, 0)
                    };
                stack.push((new_direction, (position.0 + new_direction.0, position.1 + new_direction.1)));
            },
            '\\' => {
                let new_direction = 
                    if direction.0 == 1 {
                        (0, 1)
                    } else if direction.0 == -1 {
                        (0, -1)
                    } else if direction.1 == 1 {
                        (1, 0)
                    } else {
                        (-1, 0)
                    };
                stack.push((new_direction, (position.0 + new_direction.0, position.1 + new_direction.1)));
            },
            '-' => {
                if direction.0 == 1 || direction.0 == -1 {
                    stack.push((direction, (position.0 + direction.0, position.1 + direction.1)));
                }
                else {
                    stack.push(((1, 0), (position.0 + 1, position.1)));
                    stack.push(((-1, 0), (position.0 - 1, position.1)));
                }

            },
            '|' => {
                if direction.1 == 1 || direction.1 == -1 {
                    stack.push((direction, (position.0 + direction.0, position.1 + direction.1)));
                }
                else {
                    stack.push(((0, 1), (position.0, position.1 + 1)));
                    stack.push(((0, -1), (position.0, position.1 - 1)));
                }
                

            },
            _ => panic!()
        }
    }

    HashSet::<(i64, i64)>::from_iter(visited.iter().map(|x| x.1)).len() as i64
}

fn main() {
    let input = include_str!("../input.txt");

    let grid = {
        let height = input.lines().count();
        let subgrid = input.lines().flat_map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<char>>();
        let width = subgrid.len() / height;
        Grid::new(height, width, subgrid)
    };

    let pt1 = part1(&grid, (1, 0), (0, 0));
    println!("{}", pt1);

    let mut max = pt1;

    for x in 0..grid.width as i64 {
        max = std::cmp::max(max, part1(&grid, (0, 1), (x, 0)));
        max = std::cmp::max(max, part1(&grid, (0, -1), (x, grid.height as i64 - 1)));
    }

    for y in 0..grid.height as i64 {
        max = std::cmp::max(max, part1(&grid, (1, 0), (0, y)));
        max = std::cmp::max(max, part1(&grid, (-1, 0), (grid.width as i64 - 1, y)));
    }
 
    println!("{}", max);
}
