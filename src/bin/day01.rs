use std::io;
use regex::Regex;

fn main() {
    let input = get_input();

    let part1_res = calc_part1(&input);
    println!("part1 res {part1_res:?}");

    let part2_res = calc_part2(&input);
    println!("part1 res {part2_res:?}");
}

fn calc_part1(input: &Vec<String>) -> u32 {
    let re_first_num = Regex::new(r"^.*?(\d)").unwrap();
    let re_last_num = Regex::new(r".*(\d).*?$").unwrap();

    sum_using_regexes(re_first_num, re_last_num, input)
}

fn sum_using_regexes(re_first_num: Regex, re_last_num: Regex, input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|line| {
            let (_, [first_num]) = re_first_num.captures(line).unwrap().extract();
            let (_, [last_num]) = re_last_num.captures(line).unwrap().extract();

            parse_digit(first_num) * 10 + parse_digit(last_num)
        })
        .sum()
}

fn calc_part2(input: &Vec<String>) -> u32 {
    let re_first_num = Regex::new(r"^.*?([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_last_num = Regex::new(r".*([0-9]|zero|one|two|three|four|five|six|seven|eight|nine).*?$").unwrap();

    sum_using_regexes(re_first_num, re_last_num, input)
}

fn parse_digit(str: &str) -> u32 {
    match str {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => str.parse().expect("Failed to parse digit")
    }
}

fn get_input() -> Vec<String> {
    return io::stdin().lines()
        .map(|line| line.expect("Unable to read line"))
        .collect()
}
