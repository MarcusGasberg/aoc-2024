use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let answer = reader
            .lines()
            .map_while(|line_res| {
                let Result::Ok(line) = line_res else {
                    return Option::None;
                };

                let x = line.split("   ").map(|x| x.to_string());

                Some(x.collect_vec())
            })
            .fold((vec![], vec![]), |mut acc, curr| {
                let first = curr.first().unwrap();
                let last = curr.last().unwrap();

                acc.0.push(first.parse::<usize>().unwrap());
                acc.1.push(last.parse::<usize>().unwrap());

                acc
            });
        let (mut left, mut right) = answer;

        left.sort();
        right.sort();

        Ok(std::iter::zip(left, right)
            .map(|(x, y)| x.abs_diff(y))
            .sum())
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
