use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::task::Wake;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST_1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TEST_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let answer = reader
            .lines()
            .map_while(|line_res| {
                let Result::Ok(line) = line_res else {
                    return None;
                };

                let rgx = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
                let multiplications = rgx.captures_iter(&line);

                let products = multiplications
                    .map(|captures| captures.extract())
                    .map(|(_, [x, y])| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                    .fold(0, |acc, (x, y)| acc + x * y);

                Option::Some(products)
            })
            .sum::<i32>();
        Ok(answer)
    }

    assert_eq!(161, part1(BufReader::new(TEST_1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
        let mut result = Vec::new();
        let mut last = 0;
        for (index, matched) in text.match_indices(r) {
            if last != index {
                result.push(&text[last..index]);
            }
            result.push(matched);
            last = index + matched.len();
        }
        if last < text.len() {
            result.push(&text[last..]);
        }
        result
    }

    fn part2<R: BufRead>(mut reader: R) -> Result<i32> {
        let mut command: String = "".to_string();
        reader
            .read_to_string(&mut command)
            .expect("Could not read input");

        let rgx = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        let multiplications = rgx.captures_iter(&command);

        let do_dont = Regex::new(r"do.*?\(\)").unwrap();

        let digits_rgx = Regex::new(r"\((\d+),(\d+)\)").unwrap();

        let mut last_mul_idx = 0;
        let mut product = 0;
        multiplications.for_each(|capture| {
            for ele_opt in capture.iter() {
                let Some(ele) = ele_opt else {
                    continue;
                };

                let line_slice = &command[last_mul_idx..ele.end()];
                let do_or_dont_until_start_opt = do_dont.find_iter(line_slice).collect_vec();
                let mut do_or_dont = "do";
                if let Some(do_or_dont_res_match) = do_or_dont_until_start_opt.last() {
                    let start_match = do_or_dont_res_match.start();
                    if start_match != 0 {
                        last_mul_idx = start_match;
                    }
                    if do_or_dont_res_match.as_str() == "don't()" {
                        do_or_dont = "don't";
                    }
                };

                if do_or_dont == "do" {
                    let nums = digits_rgx.captures(ele.as_str()).unwrap();
                    let first_num = nums.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let second_num = nums.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    product += first_num * second_num;
                }
            }
        });

        println!("Product: {}", product);
        Ok(product)
    }

    assert_eq!(48, part2(BufReader::new(TEST_2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
