use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i64,
    position: (i64, i64),
    direction: (i64, i64)
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {

        let partial_cmp = 
        other
            .cost
            .cmp(&self.cost);

        if partial_cmp != Ordering::Equal {
            return partial_cmp;
        }

        let mag_self = std::cmp::max(self.direction.0.abs(), self.direction.1.abs());
        let mag_other = std::cmp::max(other.direction.0.abs(), other.direction.1.abs());
        mag_other.cmp(&mag_self)
            .then_with(|| self.direction.cmp(&other.direction))
            .then_with(|| self.position.cmp(&other.position))
            
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    grid: Vec<i64>,
    width: i64,
    height: i64,
}

impl Grid {
    fn at(&self, (x, y): (i64, i64)) -> i64 {
        let w = self.width;
        if x >= w || x < 0 || y >= self.height || y < 0 {
            panic!();
        }
        self.grid[(x + w * y) as usize]
    }

    fn at_mut(&mut self, (x, y): (i64, i64)) -> &mut i64 {
        let w = self.width;
        if x >= w || x < 0 || y >= self.height || y < 0 {
            panic!();
        }

        &mut self.grid[(x + w * y) as usize]
    }
}


fn dijkstra(grid: &Grid) -> Grid {

    let start = (0, 0);
    let end = (grid.width - 1, grid.height - 1);

    let mut heap = BinaryHeap::new();

    let mut dist = Grid { grid: vec![i64::MAX; (grid.width * grid.height) as usize], width: grid.width, height: grid.height };
    *dist.at_mut(start) = 0;

    heap.push(State {
        cost: 0,
        position: start,
        direction: (0, 0)
    });

    let mut is_first_tile = true;

    while let Some(State { cost, position, direction }) = heap.pop() {
        if cost > dist.at(position) {
            continue;
        }
        
        let starting_position = (position.0 - direction.0, position.1 - direction.1);

        let mut accumulated_cost = cost;

        println!("Position: {:?}, starting position: {:?}, direction: {:?}, current cost: {}", position, starting_position, direction, cost);

        for dx in std::cmp::max(direction.0+1, 1)..=3 {

            if starting_position.0 + dx >= grid.width {
                break;
            }

            let edge = (starting_position.0 + dx, position.1);
            accumulated_cost += grid.at(edge);
            println!("Testing: {:?}, Cost: {}, accumulation: {}", edge, grid.at(edge), accumulated_cost);

            let next = State {
                cost: accumulated_cost,
                position: edge,
                direction: (dx, 0)
            };

            if !(next.cost > dist.at(next.position)) {
                println!("Adding");
                heap.push(next);
                *dist.at_mut(next.position) = next.cost;
            }
        }

        accumulated_cost = cost;
        for dx in (-3..=std::cmp::min(direction.0-1, -1)).rev() {

            if starting_position.0 + dx < 0 {
                break;
            }

            let edge = (starting_position.0 + dx, position.1);
            accumulated_cost += grid.at(edge);
            println!("Testing: {:?}, Cost: {}, accumulation: {}", edge, grid.at(edge), accumulated_cost);

            let next = State {
                cost: accumulated_cost,
                position: edge,
                direction: (dx, 0)
            };

            if next.cost < dist.at(next.position) {
                println!("Adding");
                heap.push(next);
                *dist.at_mut(next.position) = next.cost;
            }
        }

        accumulated_cost = cost;

        for dy in std::cmp::max(direction.1+1, 1)..=3 {
            if starting_position.1 + dy >= grid.height {
                break;
            }

            let edge = (position.0, starting_position.1 + dy);
            accumulated_cost += grid.at(edge);
            println!("Testing: {:?}, Cost: {}, accumulation: {}", edge, grid.at(edge), accumulated_cost);

            let next = State {
                cost: accumulated_cost,
                position: edge,
                direction: (0, dy)
            };

            if next.cost < dist.at(next.position) {
                println!("Adding");
                heap.push(next);
                *dist.at_mut(next.position) = next.cost;
            }
        }

        accumulated_cost = cost;

        for dy in (-3..=std::cmp::min(direction.1-1, -1)).rev() {
            if starting_position.1 + dy < 0 {
                break;
            }

            let edge = (position.0, starting_position.1 + dy);
            accumulated_cost += grid.at(edge);
            println!("Testing: {:?}, Cost: {}, accumulation: {}", edge, grid.at(edge), accumulated_cost);

            let next = State {
                cost: accumulated_cost,
                position: edge,
                direction: (0, dy)
            };

            if next.cost < dist.at(next.position) {
                println!("Adding");
                heap.push(next);
                *dist.at_mut(next.position) = next.cost;
            }
        }

        for y in 0..grid.height as usize {
            println!("{:?}", dist.grid[y*grid.width as usize .. (y+1)*grid.width as usize].iter().map(|x| if *x == 9223372036854775807 { 0 } else { *x }).collect::<Vec<i64>>());
        }
    }


    dist
}

fn main() {
    let input = include_str!("../input.txt");

    let grid = {
        let height = input.lines().count();
        let subgrid = input.lines().flat_map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as i64).collect::<Vec<i64>>()).collect::<Vec<i64>>();
        let width = subgrid.len() / height;
        Grid { grid: subgrid, width: width as i64, height: height as i64 }
    };

    let dist = dijkstra(&grid);
}
