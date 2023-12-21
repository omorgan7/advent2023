use std::vec::Vec;

#[derive(Clone)]
struct Grid {
    grid: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn at(&self, (x, y): (i64, i64)) -> char {
        let w = self.width as i64;
        if x >= w || x < 0 || y >= self.height as i64 || y < 0 {
            panic!();
        }
        self.grid[(x + w * y) as usize]
    }

    fn at_mut(&mut self, (x, y): (i64, i64)) -> &mut char {
        let w = self.width as i64;
        if x >= w || x < 0 || y >= self.height as i64 || y < 0 {
            panic!();
        }
        &mut self.grid[(x + w * y) as usize]
    }
}

fn spin_north(mgrid: &mut Grid) {
    for y in 1..mgrid.height as i64 {
        for x in 0..mgrid.width as i64 {
            let p0 = mgrid.at((x, y));
            if p0 != 'O' {
                continue;
            }

            *mgrid.at_mut((x, y)) = '.';

            let mut final_rest = y;
            loop {
                if final_rest - 1 == 0 {
                    if mgrid.at((x, final_rest - 1)) == '.' {
                        final_rest -= 1;
                    }

                    break;
                }

                if mgrid.at((x, final_rest - 1)) != '.' {
                    break;
                }

                final_rest -= 1;
            }

            *mgrid.at_mut((x, final_rest)) = 'O';
        }
    }
}

fn spin_east(mgrid: &mut Grid) {
    for x in (0..(mgrid.width as i64) - 1).rev() {
        for y in 0..mgrid.height as i64 {
            let p0 = mgrid.at((x, y));
            if p0 != 'O' {
                continue;
            }

            *mgrid.at_mut((x, y)) = '.';

            let mut final_rest = x;
            loop {
                if final_rest + 1 == (mgrid.width - 1) as i64 {
                    if mgrid.at((final_rest + 1, y)) == '.' {
                        final_rest += 1;
                    }

                    break;
                }

                if mgrid.at((final_rest + 1, y)) != '.' {
                    break;
                }

                final_rest += 1;
            }

            *mgrid.at_mut((final_rest, y)) = 'O';
        }
    }
}

fn spin_west(mgrid: &mut Grid) {
    for x in 1..mgrid.width as i64 {
        for y in 0..mgrid.height as i64 {
            let p0 = mgrid.at((x, y));
            // println!("{:?}", (x, y));
            if p0 != 'O' {
                continue;
            }

            *mgrid.at_mut((x, y)) = '.';

            let mut final_rest = x;
            loop {
                if final_rest - 1 == 0 {
                    if mgrid.at((final_rest - 1, y)) == '.' {
                        final_rest -= 1;
                    }

                    break;
                }

                if mgrid.at((final_rest - 1, y)) != '.' {
                    break;
                }

                final_rest -= 1;
            }

            *mgrid.at_mut((final_rest, y)) = 'O';
        }
    }
}

fn spin_south(mgrid: &mut Grid) {
    for y in (0..(mgrid.height as i64) - 1).rev() {
        for x in 0..mgrid.width as i64 {
            let p0 = mgrid.at((x, y));
            if p0 != 'O' {
                continue;
            }

            *mgrid.at_mut((x, y)) = '.';

            let mut final_rest = y;
            loop {
                if final_rest + 1 == (mgrid.height - 1) as i64 {
                    if mgrid.at((x, final_rest + 1)) == '.' {
                        final_rest += 1;
                    }

                    break;
                }

                if mgrid.at((x, final_rest + 1)) != '.' {
                    break;
                }

                final_rest += 1;
            }

            *mgrid.at_mut((x, final_rest)) = 'O';
        }
    }
}

fn compute_weight(mgrid: &Grid) -> i64 {
    let mut sum = 0;
    for y in 0..mgrid.height as i64 {
        let mut o_count: i64 = 0;
        for x in 0..mgrid.width as i64 {
            if mgrid.at((x, y)) == 'O' {
                o_count += 1;
            }
        }

        sum += o_count * (mgrid.height as i64 - y);
    }
    sum
}

fn main() {
    let input = include_str!("../input.txt");

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let grid = Grid {
        grid: input
            .lines()
            .flat_map(|l| l.chars().collect::<Vec<char>>())
            .collect(),
        width,
        height,
    };

    let mut mgrid = grid.clone();
    spin_north(&mut mgrid);

    println!("{}", compute_weight(&mgrid));

    mgrid = grid.clone();

    let mut last_grids = (0..200)
        .map(|i| (i, mgrid.clone()))
        .collect::<Vec<(usize, Grid)>>();
    let mut cycle_start: Option<usize> = None;
    let mut cycle_length = 0;
    let mut should_stop = false;
    let mut extra_iters = last_grids.len();

    let iteration_count = 1000000000;
    for c in 0..iteration_count {
        spin_north(&mut mgrid);
        spin_west(&mut mgrid);
        spin_south(&mut mgrid);
        spin_east(&mut mgrid);

        let len = last_grids.len();

        for i in 0..last_grids.len() {
            if i == (c % last_grids.len()) {
                continue;
            }

            let last_grid = &last_grids[i];
            if last_grid.1.grid == mgrid.grid {
                if cycle_start.is_none() {
                    cycle_start = Some(c % last_grids.len());
                    cycle_length = (c % last_grids.len()) - i;
                    extra_iters = cycle_length;
                }

                extra_iters -= 1;
                if extra_iters == 0 {
                    should_stop = true;
                }

                break;
            }
        }

        if should_stop {
            break;
        }

        last_grids[c % len] = (c, mgrid.clone());
    }

    let offset = (iteration_count - cycle_start.unwrap() - 1) % cycle_length;
    mgrid = last_grids[(cycle_start.unwrap() + offset) % last_grids.len()]
        .1
        .clone();

    println!("{}", compute_weight(&mgrid));
}
