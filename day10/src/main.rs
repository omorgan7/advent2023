use std::collections::HashSet;
use std::vec::Vec;

struct Grid {
    grid: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn find_s(&self) -> (i64, i64) {
        let index = self.grid.iter().position(|x| *x == 'S').unwrap();
        ((index % self.width) as i64, (index / self.width) as i64)
    }

    fn at(&self, (x, y): (i64, i64)) -> char {
        let w = self.width as i64;
        if x >= w || x < 0 {
            return '.';
        }
        if y >= self.height as i64 || y < 0 {
            return '.';
        }
        self.grid[(x + w * y) as usize]
    }

    fn neighbours((x, y): (i64, i64)) -> [(i64, i64); 4] {
        [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
    }

    fn is_connected_to_start((sx, sy): (i64, i64), (x, y): (i64, i64), pipe: char) -> bool {
        match pipe {
            '|' => (x - sx) == 0 && (y - sy).abs() == 1,
            '-' => (x - sx).abs() == 1 && (y - sy) == 0,
            'L' => (x - sx) == -1 && (y - sy) == 0 || (x - sx) == 0 && (y - sy) == 1,
            'J' => (x - sx) == 1 && (y - sy) == 0 || (x - sx) == 0 && (y - sy) == 1,
            '7' => (x - sx) == 1 && (y - sy) == 0 || (x - sx) == 0 && (y - sy) == -1,
            'F' => (x - sx) == -1 && (y - sy) == 0 || (x - sx) == 0 && (y - sy) == -1,
            _ => false,
        }
    }

    fn connections((x, y): (i64, i64), pipe: char) -> [(i64, i64); 2] {
        match pipe {
            '|' => [(x, y + 1), (x, y - 1)],
            '-' => [(x + 1, y), (x - 1, y)],
            'L' => [(x, y - 1), (x + 1, y)],
            'J' => [(x, y - 1), (x - 1, y)],
            '7' => [(x, y + 1), (x - 1, y)],
            'S' => [(x + 1, y), (x, y + 1)],
            'F' => [(x + 1, y), (x, y + 1)],
            _ => panic!("{:?}, {}", (x, y), pipe),
        }
    }
}

fn part2(grid: &Grid) -> i64 {
    let start = grid.find_s();

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut s0 = Vec::<(i64, i64)>::new();
    let mut s1 = Vec::<(i64, i64)>::new();

    for neighbour in Grid::neighbours(start) {
        if Grid::is_connected_to_start(start, neighbour, grid.at(neighbour)) {
            if s0.is_empty() {
                s0.push(neighbour);
            } else if s1.is_empty() {
                s1.push(neighbour)
            } else {
                panic!();
            }
        }
    }

    loop {
        let p0 = s0.pop().unwrap();
        let p1 = s1.pop().unwrap();

        visited.insert(p0);
        visited.insert(p1);

        if p0 == p1 {
            break;
        }

        for neighbour in Grid::connections(p0, grid.at(p0)) {
            if !visited.contains(&neighbour) {
                s0.push(neighbour);
            }
        }

        for neighbour in Grid::connections(p1, grid.at(p1)) {
            if !visited.contains(&neighbour) {
                s1.push(neighbour);
            }
        }
    }

    let mut inside_count = 0;

    for y in 0..grid.height as i64 {
        let mut outside = true;

        let mut is_f = false;
        let mut is_l = false;

        for x in 0..grid.width as i64 {
            let p0 = (x, y);

            let is_pipe = visited.contains(&p0);
            let p = grid.at(p0);
            if is_pipe {
                if p == '|' {
                    outside = !outside;
                }

                if p == 'F' {
                    is_f = true;
                }

                if p == 'J' {
                    if is_l {
                        is_l = false;
                    }
                    if is_f {
                        is_f = false;
                        outside = !outside;
                    }
                }

                if p == 'L' {
                    is_l = true;
                }

                if p == '7' {
                    if is_f {
                        is_f = false;
                    }
                    if is_l {
                        is_l = false;
                        outside = !outside;
                    }
                }
            }

            if !is_pipe && !outside {
                inside_count += 1;
            }
        }
    }

    inside_count
}

fn part1(grid: &Grid) -> i64 {
    let start = grid.find_s();

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut s0 = Vec::<(i64, i64)>::new();
    let mut s1 = Vec::<(i64, i64)>::new();

    for neighbour in Grid::neighbours(start) {
        if Grid::is_connected_to_start(start, neighbour, grid.at(neighbour)) {
            if s0.is_empty() {
                s0.push(neighbour);
            } else if s1.is_empty() {
                s1.push(neighbour)
            } else {
                panic!();
            }
        }
    }

    let mut depth = 0;
    loop {
        depth += 1;
        let p0 = s0.pop().unwrap();
        let p1 = s1.pop().unwrap();

        if p0 == p1 {
            break;
        }

        visited.insert(p0);
        visited.insert(p1);

        for neighbour in Grid::connections(p0, grid.at(p0)) {
            if !visited.contains(&neighbour) {
                s0.push(neighbour);
            }
        }

        for neighbour in Grid::connections(p1, grid.at(p1)) {
            if !visited.contains(&neighbour) {
                s1.push(neighbour);
            }
        }
    }

    depth
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

    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}
