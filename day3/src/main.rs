use std::vec::Vec;

struct Grid {
    grid: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(height: usize, width: usize) -> Grid {
        let mut grid = Vec::<char>::new();
        grid.resize(width * height, '.');
        Grid {
            grid,
            width,
            height,
        }
    }

    fn at(&self, x: i64, y: i64) -> char {
        if x < 0 || (x as usize) >= self.width {
            return '.';
        }
        if y < 0 || (y as usize) >= self.height {
            return '.';
        }

        self.grid[(x as usize) + self.width * (y as usize)]
    }

    fn neighbours(x: i64, y: i64) -> [(i64, i64); 8] {
        [
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
            (x + 1, y + 1),
            (x - 1, y + 1),
            (x + 1, y - 1),
            (x - 1, y - 1),
        ]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut char {
        &mut self.grid[x + self.width * y]
    }
}

fn part1(grid: &Grid) -> i64 {
    let mut sum: i64 = 0;
    let mut accumulator: i64 = 0;
    let mut is_part_number = false;
    let mut power: i64 = 1;

    for y in (0..grid.height as i64).rev() {
        for x in (0..grid.width as i64).rev() {
            if !grid.at(x, y).is_ascii_digit() {
                if is_part_number {
                    println!("{}", accumulator);
                    sum += accumulator;
                }
                is_part_number = false;
                accumulator = 0;
                power = 1;
                continue;
            }

            accumulator += power * (grid.at(x, y).to_digit(10).unwrap() as i64);
            power *= 10;

            if Grid::neighbours(x, y).iter().any(|(nx, ny)| {
                let v = grid.at(*nx, *ny);
                !v.is_ascii_digit() && v != '.'
            }) {
                is_part_number = true;
            }
        }
        if is_part_number {
            println!("{}", accumulator);
            sum += accumulator;
        }
        is_part_number = false;
        accumulator = 0;
        power = 1;
    }
    if is_part_number {
        sum += accumulator;
    }

    sum
}

fn accumulate(grid: &Grid, indices: &[(i64, i64)]) -> i64 {
    let mut accumulator: i64 = 0;
    let mut power: i64 = 1;

    for (x, y) in indices {
        accumulator += power * (grid.at(*x, *y).to_digit(10).unwrap() as i64);
        power *= 10;
    }

    accumulator
}

fn part2(grid: &Grid) -> i64 {
    let mut number_indices = Vec::<(i64, i64)>::new();
    let _gear_indices = Vec::<(i64, i64)>::new();
    let mut sum = 0;

    let mut gear_links = Vec::<(i64, i64)>::new();

    for y in (0..grid.height as i64).rev() {
        for x in (0..grid.width as i64).rev() {
            if !grid.at(x, y).is_ascii_digit() {
                if !number_indices.is_empty() {
                    for (ix, iy) in number_indices.iter() {
                        let mut need_break = false;
                        for (nx, ny) in Grid::neighbours(*ix, *iy).iter() {
                            if grid.at(*nx, *ny) == '*' {
                                for (nnx, nny) in Grid::neighbours(*nx, *ny).iter() {
                                    if grid.at(*nnx, *nny).is_ascii_digit()
                                        && !number_indices.contains(&(*nnx, *nny))
                                        && !gear_links.contains(&(*nx, *ny))
                                    {
                                        gear_links.push((*nx, *ny));
                                        need_break = true;
                                        break;
                                    }
                                }
                            }
                            if need_break {
                                break;
                            }
                        }
                        if need_break {
                            break;
                        }
                    }
                }
                number_indices.clear();
            } else {
                number_indices.push((x, y));
            }
        }
    }
    for (gx, gy) in gear_links {
        let mut n0: Option<(i64, i64)> = None;
        let mut n1: Option<(i64, i64)> = None;

        for (nx, ny) in Grid::neighbours(gx, gy).iter() {
            if grid.at(*nx, *ny).is_ascii_digit() {
                if n0.is_none() {
                    n0 = Some((*nx, *ny));
                } else {
                    let n = n0.unwrap();
                    if !Grid::neighbours(n.0, n.1).contains(&(*nx, *ny)) {
                        n1 = Some((*nx, *ny));
                    }
                }
            }
        }

        let get_indices = |n| {
            let mut n_indices = Vec::<(i64, i64)>::new();
            n_indices.push(n);

            let mut x_offset = 1;
            loop {
                if !grid.at(n.0 + x_offset, n.1).is_ascii_digit() {
                    break;
                }
                n_indices.push((n.0 + x_offset, n.1));
                x_offset += 1;
            }
            x_offset = -1;
            loop {
                if !grid.at(n.0 + x_offset, n.1).is_ascii_digit() {
                    break;
                }
                n_indices.push((n.0 + x_offset, n.1));
                x_offset -= 1;
            }

            n_indices.sort_by(|a, b| a.0.cmp(&b.0).reverse());
            n_indices
        };

        sum += accumulate(grid, &get_indices(n0.unwrap())) * accumulate(grid, &get_indices(n1.unwrap()));
    }

    sum
}

fn main() {
    let input = include_str!("../input.txt");

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut grid = Grid::new(height, width);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            *grid.at_mut(x, y) = c;
        }
    }

    println!("{}", part1(&grid));
    // println!("{}", part2(&grid));
}
