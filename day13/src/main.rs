use std::vec::Vec;

#[derive(Clone)]
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

    fn at(&self, x: i64, y: i64) -> char {
        if x < 0 || (x as usize) >= self.width {
            panic!();
        }
        if y < 0 || (y as usize) >= self.height {
            panic!();
        }

        self.grid[(x as usize) + self.width * (y as usize)]
    }

    fn at_mut(&mut self, x: i64, y: i64) -> &mut char {
        if x < 0 || (x as usize) >= self.width {
            panic!();
        }
        if y < 0 || (y as usize) >= self.height {
            panic!();
        }

        &mut self.grid[(x as usize) + self.width * (y as usize)]
    }
}



fn part1(grid: &Grid, skip: &Option<(i64, i64)>) -> Option<(i64, i64, i64)> {
    let mut mirror_line : Option<i64> = None;

    for x in 0..grid.width as i64 {
        let mut all_mirrored : Option<bool> = None;

        if skip.is_some() {
            if skip.unwrap().0 == x + 1 { 
                continue;
            }
        }

        for offset in 1..grid.width as i64 {

            if x - offset + 1 < 0 || x + offset >= grid.width as i64 {
                break;
            }

            all_mirrored = Some(true);
            if grid.at(x - offset + 1, 0) == grid.at(x + offset, 0) {   
                for y in 1..grid.height as i64 {
                    if grid.at(x - offset + 1, y) != grid.at(x + offset, y) {
                        all_mirrored = Some(false);
                        break;
                    }
                }
            }
            else {
                all_mirrored = Some(false);
            }

            if !all_mirrored.unwrap() {
                break;
            }
        }

        if all_mirrored.unwrap_or(false) {
            mirror_line = Some(x+1);
            break;
        }
    }

    if mirror_line.is_some() {
        return Some((mirror_line.unwrap(), mirror_line.unwrap(), -10));
    }

    for y in 0..grid.height as i64 {
        let mut all_mirrored : Option<bool> = None;

        if skip.is_some() {
            if skip.unwrap().1 == y + 1 { 
                continue;
            }
        }

        for offset in 1..grid.height as i64 {
            if y - offset + 1 < 0 || y + offset >= grid.height as i64 {
                break;
            }

            all_mirrored = Some(true);
            if grid.at(0, y - offset + 1) == grid.at(0, y + offset) {   
                for x in 1..grid.width as i64 {
                    if grid.at(x, y - offset + 1) != grid.at(x, y + offset) {
                        all_mirrored = Some(false);
                        break;
                    }
                }
            }
            else {
                all_mirrored = Some(false);
            }

            if !all_mirrored.unwrap() {
                break;
            }
        }

        if all_mirrored.unwrap_or(false) {
            mirror_line = Some(y+1);
            break;
        }
    }

    if mirror_line.is_none() {
        return None;
    }
    Some((100 * mirror_line.unwrap(), -10, mirror_line.unwrap()))
}

fn main() {
    let input = include_str!("../input.txt");

    let grids = input.split("\n\n").map(|grid| {
        let height = grid.lines().count();
        let subgrid = grid.lines().flat_map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<char>>();
        let width = subgrid.len() / height;
        Grid::new(height, width, subgrid)
    }).collect::<Vec<Grid>>();

   let pt1 = grids.iter().map(|grid| part1(grid, &None).unwrap().0).sum::<i64>();

   let pt2 = grids.iter().map(|grid| {

        let mut mutated_grid = grid.clone();

        let original_reflection = part1(&mutated_grid, &None).unwrap();

        let skip = (original_reflection.1, original_reflection.2);

        for y in 0..grid.height as i64 {
            for x in 0..grid.width as i64 {
                *mutated_grid.at_mut(x, y) = if grid.at(x, y) == '.' { '#' } else { '.' };
                let possible_reflection = part1(&mutated_grid, &Some(skip));
                if possible_reflection.is_some() {
                    return possible_reflection.unwrap().0;
                }
                *mutated_grid.at_mut(x, y) = grid.at(x, y);
            }
        }

        panic!();
    }).sum::<i64>();

    println!("{}", pt1);
    println!("{}", pt2);
}
