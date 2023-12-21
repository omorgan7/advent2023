use std::vec::Vec;

struct Grid {
    grid: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
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

    fn empty_column_count(&self, start: i64, end: i64) -> i64 {
        let mut count = 0;

        let sx = std::cmp::min(start, end);
        let ex = std::cmp::max(start, end);
        for x in sx..ex {
            let mut is_empty = true;
            for y in 0..self.height {
                if self.at((x, y as i64)) != '.' {
                    is_empty = false;
                    break;
                }
            }
            if is_empty {
                count += 1;
            }
        }

        count
    }

    fn empty_row_count(&self, start: i64, end: i64) -> i64 {
        let mut count = 0;

        let sy = std::cmp::min(start, end);
        let ey = std::cmp::max(start, end);

        for y in sy..ey {
            let mut is_empty = true;
            for x in 0..self.width {
                if self.at((x as i64, y)) != '.' {
                    is_empty = false;
                    break;
                }
            }
            if is_empty {
                count += 1;
            }
        }

        count
    }
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

    let mut galaxies = Vec::new();

    for y in 0..grid.height as i64 {
        for x in 0..grid.width as i64 {
            if grid.at((x, y)) == '#' {
                galaxies.push((x, y));
            }
        }
    }

    let mut distances = Vec::<i64>::new();

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let dx = (galaxies[j].0 - galaxies[i].0).abs()
                + grid.empty_column_count(galaxies[i].0, galaxies[j].0);
            let dy = (galaxies[j].1 - galaxies[i].1).abs()
                + grid.empty_row_count(galaxies[i].1, galaxies[j].1);

            distances.push(dx + dy);
        }
    }

    println!("{}", distances.iter().sum::<i64>());

    distances.clear();

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let dx = (galaxies[j].0 - galaxies[i].0).abs()
                + (1000000 - 1) * grid.empty_column_count(galaxies[i].0, galaxies[j].0);
            let dy = (galaxies[j].1 - galaxies[i].1).abs()
                + (1000000 - 1) * grid.empty_row_count(galaxies[i].1, galaxies[j].1);

            distances.push(dx + dy);
        }
    }

    println!("{}", distances.iter().sum::<i64>());
}
