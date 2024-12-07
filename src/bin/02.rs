use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{Itertools, TupleWindows};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    fn get_direction(val: (usize, usize)) -> i32 {
        match val.0.cmp(&val.1) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        }
    }

    fn count_errors(windows: Vec<(usize, usize)>) -> usize {
        let mut direction = 0;
        let mut count = 0;
        for (x, y) in windows {
            if x.abs_diff(y) > 3 {
                count += 1;
                continue;
            }

            let window_dir = get_direction((x, y));
            if window_dir == 0 {
                count += 1;
                continue;
            }

            if direction == 0 {
                direction = window_dir;
                continue;
            }

            if window_dir != direction {
                count += 1;
            }
        }

        count
    }

    fn get_windows(line: &str) -> Vec<(usize, usize)> {
        line.split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .tuple_windows()
            .collect::<Vec<(usize, usize)>>()
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map_while(|line_res| {
                let Result::Ok(line) = line_res else {
                    return Option::None;
                };

                let windows = get_windows(&line);
                println!("{:?}", windows);

                let errors = count_errors(windows);

                Option::Some(errors == 0)
            })
            .filter(|&x| x)
            .count();
        Ok(answer)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map_while(|line_res| {
                let Result::Ok(line) = line_res else {
                    return Option::None;
                };

                let nums = line
                    .split(" ")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_vec();

                let mutations = (0..nums.len()).map(|i| {
                    let mut nums = nums.clone();
                    nums.remove(i);

                    nums
                });

                let no_errors = mutations
                    .map(|mutation| {
                        let windows = mutation
                            .iter()
                            .tuple_windows()
                            .map(|(&x, &y)| (x, y))
                            .collect::<Vec<(usize, usize)>>();

                        count_errors(windows)
                    })
                    .filter(|&x| x == 0)
                    .count();

                Some(no_errors != 0)
            })
            .filter(|&x| x)
            .count();

        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
