fn main() {
    let input = include_str!("../input.txt");

    let part1 = input.lines().fold(0, |sum, line| {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;

        for c in line.chars() {
            if c.is_digit(10) {
                if first.is_none() {
                    first = Some(c);
                }
                last = Some(c);
            }
        }

        sum + (first.unwrap().to_string() + &last.unwrap().to_string())
            .parse::<i64>()
            .unwrap()
    });

    println!("{}", part1);

    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let part2 = input.lines().fold(0, |sum, line| {
        let mut first: Option<(String, usize)> = None;
        let mut last: Option<(String, usize)> = None;

        let mut search = |arr: &[&str]| {
            for w in arr {
                let mut search = line.match_indices(w);

                let first_search = search.next();
                let last_search = search.last();

                if first_search.is_none() {
                    continue;
                }
                let (first_index, _) = first_search.unwrap();
                let (last_index, _) = last_search.unwrap_or((first_index, w));

                if first.is_none() || first.as_ref().unwrap().1 > first_index {
                    first = Some((w.to_string(), first_index))
                }
                if last.is_none() || last.as_ref().unwrap().1 < last_index {
                    last = Some((w.to_string(), last_index))
                }
            }
        };

        search(&digits);
        search(&words);

        let str_to_digit = |d: &str, arr: &[&str]| arr.iter().position(|&r| r == d).map(|c| c + 1);

        let first_number = 10
            * match str_to_digit(&first.as_ref().unwrap().0, &words) {
                None => str_to_digit(&first.as_ref().unwrap().0, &digits).unwrap(),
                Some(c) => c,
            };

        let last_number = match str_to_digit(&last.as_ref().unwrap().0, &words) {
            None => str_to_digit(&last.as_ref().unwrap().0, &digits).unwrap(),
            Some(c) => c,
        };

        sum + first_number + last_number
    });

    println!("{}", part2);
}
