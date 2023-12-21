use rustc_hash::FxHashMap as HashMap;
use std::ops::Range;
use std::vec::Vec;

fn main() {
    let input = include_str!("../input.txt");

    let seeds: Vec<i64> = input.lines().next().unwrap()[6..]
        .split(' ')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let mut line_it = input.lines().skip(2);

    let collect_map = |it: &mut std::iter::Skip<std::str::Lines<'_>>| {
        let mut map = HashMap::<Range<i64>, Range<i64>>::default();
        loop {
            let line = it.next();
            if line.is_none() || line.unwrap().is_empty() {
                break;
            }

            let mut numbers = line
                .unwrap()
                .split(' ')
                .filter_map(|s| s.parse::<i64>().ok());
            let destination = numbers.next();
            let source = numbers.next();
            let range = numbers.next();

            if destination.is_none() || source.is_none() || range.is_none() {
                continue;
            }

            let source = source.unwrap();
            let destination = destination.unwrap();
            let range = range.unwrap() + 1;

            map.insert(source..source + range, destination..destination + range);
        }
        map
    };

    let maps = {
        let mut vec = Vec::<HashMap<Range<i64>, Range<i64>>>::new();
        loop {
            vec.push(collect_map(&mut line_it));

            if (&mut line_it).peekable().peek_mut().is_none() {
                break;
            }
        }
        vec
    };

    let mut min: Option<i64> = None;
    for s in &seeds {
        let location = maps.iter().fold(*s, |curr, map| {
            let mut value: Option<i64> = None;
            for range in map.keys() {
                if range.contains(&curr) {
                    let offset = curr - range.start;
                    value = Some(map.get(range).unwrap().start + offset);
                    break;
                }
            }
            value.unwrap_or(curr)
        });

        if min.is_none() {
            min = Some(location);
        } else {
            min = Some(std::cmp::min(min.unwrap(), location));
        }
    }

    println!("{}", min.unwrap());

    let seeds_ranges = seeds
        .chunks(2)
        .map(|c| c[0]..c[0] + c[1] + 1)
        .collect::<Vec<Range<i64>>>();
    let reverse_maps: Vec<HashMap<Range<i64>, Range<i64>>> = maps
        .iter()
        .rev()
        .map(|map| {
            let mut rev = HashMap::<Range<i64>, Range<i64>>::default();
            for (k, v) in map.iter() {
                rev.insert(v.clone(), k.clone());
            }
            rev
        })
        .collect();

    let mut location = 0;
    let mut break_outer = false;
    loop {
        let seed = reverse_maps.iter().fold(location, |curr, map| {
            let mut value: Option<i64> = None;
            for range in map.keys() {
                if range.contains(&curr) {
                    let offset = curr - range.start;
                    value = Some(map.get(range).unwrap().start + offset);
                    break;
                }
            }
            value.unwrap_or(curr)
        });

        for s in &seeds_ranges {
            if s.contains(&seed) {
                break_outer = true;
                break;
            }
        }

        if break_outer {
            break;
        }
        location += 1;
    }
    println!("{}", location);
    // min = None;
    // for r in seeds_ranges {
    //     for s in r {
    //         let location = maps.iter().fold(s, |curr, map| {
    //             let mut value : Option<i64> = None;
    //             for range in map.keys() {
    //                 if range.contains(&curr) {
    //                     let offset = curr - range.start;
    //                     value = Some(map.get(range).unwrap().start + offset);
    //                     break;
    //                 }
    //             }
    //             value.unwrap_or(curr)
    //         });
    
    //         if min.is_none() {
    //             min = Some(location);
    //         }
    //         else {
    //             min = Some(std::cmp::min(min.unwrap(), location));
    //         }
    //     }
    // }
    // println!("{}", min.unwrap());
}
