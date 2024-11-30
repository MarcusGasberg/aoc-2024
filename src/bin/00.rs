use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DAY: &str = "00";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map_while(|str_res| {
                let digits = match str_res {
                    Result::Ok(str) => {
                        Some(str.chars().filter(|&x| char::is_digit(x, 10)).collect_vec())
                    }
                    Result::Err(_) => None,
                };

                match digits {
                    Some(mut digs) => {
                        let mut first_digit = digs.first_mut()?.to_string();
                        let last_digit = digs.last()?.to_string();

                        first_digit.push_str(&last_digit);

                        Some(first_digit.parse::<usize>().unwrap())
                    }
                    None => None,
                }
            })
            .sum();

        Ok(answer)
    }

    assert_eq!(142, part1(BufReader::new(TEST.as_bytes()))?);

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
