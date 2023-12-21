use std::{thread, time};
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Clone)]
struct Grid {
    grid: Vec<char>,
    width: usize,
    height: usize,
    x_offset: i64,
    y_offset: i64
}

impl Grid {
    fn at(&self, (x, y): (i64, i64)) -> char {
        let w = self.width as i64;

        let xx = x - self.x_offset;
        let yy = y - self.y_offset;

        if xx >= w || xx < 0 || yy >= self.height as i64 || yy < 0 {
            panic!();
        }
        self.grid[(xx + w * yy) as usize]
    }

    fn at_mut(&mut self, (x, y): (i64, i64)) -> &mut char {
        let w = self.width as i64;

        let xx = x - self.x_offset;
        let yy = y - self.y_offset;

        if xx >= w || xx < 0 || yy >= self.height as i64 || yy < 0 {
            panic!("{} {} {} {}", xx, yy, w, self.height);
        }
        &mut self.grid[(xx + w * yy) as usize]
    }

    fn in_bounds(&self, (x, y): (i64, i64)) -> bool {
        let w = self.width as i64;

        let xx = x - self.x_offset;
        let yy = y - self.y_offset;

        if xx >= w || xx < 0 || yy >= self.height as i64 || yy < 0 {
            false
        }
        else {
            true
        }
    }

    fn render(&self)
    {
        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        for y in 0..self.height {
            println!("{}", self.grid[y*self.width..(y+1)*self.width].into_iter().collect::<String>());
        }
        // thread::sleep(time::Duration::from_millis(10));
    }
}

fn main() {
    let input = include_str!("../input.txt").lines().map(|line| {
        let mut it = line.split(' ');

        let direction = it.next().unwrap().chars().next().unwrap();
        let dig = it.next().unwrap().parse::<i64>().unwrap();

        let colour_code = {
            let full_code = it.next().unwrap();
            full_code[2..full_code.len()-1].to_string()
        };

        (direction, dig, colour_code)
    }).collect::<Vec<(char, i64, String)>>();

    let mut max_extent_x = 0;
    let mut min_extent_x = 0;
    let mut max_extent_y = 0;
    let mut min_extent_y = 0;

    let mut x = 0;
    let mut y = 0;
    for i in &input {
        if i.0 == 'R' || i.0 == 'L' {
            x += i.1 * if i.0 == 'R' { 1 } else { -1 };
            // println!("x: {}", x);
            max_extent_x = std::cmp::max(max_extent_x, x);
            min_extent_x = std::cmp::min(min_extent_x, x);
        }
        if i.0 == 'U' || i.0 == 'D' {
            y += i.1 * if i.0 == 'D' { 1 } else { -1 };
            // println!("y: {}", y);
            max_extent_y = std::cmp::max(max_extent_y, y);
            min_extent_y = std::cmp::min(min_extent_y, y);
        }
        // println!("{} {} {} {}", min_extent_x, max_extent_x, min_extent_y, max_extent_y);
    }

    min_extent_x -= 1;
    min_extent_y -= 1;

    let width = 2 + max_extent_x - min_extent_x;
    let height = 2 + max_extent_y - min_extent_y;

    let mut grid = Grid { grid: vec!['.'; (height * width) as usize], height: height as usize, width: width as usize, x_offset: min_extent_x, y_offset: min_extent_y };

    x = 0;
    y = 0;

    *grid.at_mut((x, y)) = '#';
    for i in &input {

        let mut new_x = x;
        let mut new_y = y;
        if i.0 == 'R' || i.0 == 'L' {
            new_x += i.1 * if i.0 == 'R' { 1 } else { -1 };
        }

        if i.0 == 'U' || i.0 == 'D' {
            new_y += i.1 * if i.0 == 'D' { 1 } else { -1 };
        }

        let x_min = std::cmp::min(x, new_x);
        let x_max = std::cmp::max(x, new_x);
        let y_min = std::cmp::min(y, new_y);
        let y_max = std::cmp::max(y, new_y);

        for yy in y_min..=y_max {
            for xx in x_min..=x_max {
                *grid.at_mut((xx, yy)) = '#';
            }
        }
        x = new_x;
        y = new_y;
    }


    let mut queue = Vec::new();

    queue.push((min_extent_x, min_extent_y));

    let mut outside_count = 1;

    let n = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1)
    ];

    let mut mgrid = grid.clone();


    let mut count = 0;

    while queue.is_empty() == false {

        let p = queue.pop().unwrap();
        *mgrid.at_mut(p) = 'X';

        for (dx, dy) in n {
            let d = (p.0 + dx, p.1 + dy);
            if mgrid.in_bounds(d) {
                if mgrid.at(d) == '.' {
                    queue.push(d);
                }
            }
        }
    }

    let actual_instructions = input.iter().map(|i| {
        let inst = match i.2.chars().nth(5).unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => panic!()
        };
        (inst, i64::from_str_radix(&i.2[0..5], 16).unwrap())
    }).collect::<Vec<(char, i64)>>();


    x = 0;
    y = 0;


    let mut area = 0;

    let mut perimeter = 0;

    for i in &actual_instructions {

        let mut new_x = x;
        let mut new_y = y;
        if i.0 == 'R' || i.0 == 'L' {
            new_x += i.1 * if i.0 == 'R' { 1 } else { -1 };
        }

        if i.0 == 'U' || i.0 == 'D' {
            new_y += i.1 * if i.0 == 'D' { 1 } else { -1 };
        }

        area += x * new_y - y * new_x;
        perimeter += i.1;

        x = new_x;
        y = new_y;
    }

    println!("{}", area.abs() / 2 + perimeter / 2 + 1);


}
