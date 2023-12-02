use std::collections::HashMap;
use std::io;
use regex::Regex;

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<RoundRetrieve>,
}

#[derive(Debug)]
struct RoundRetrieve {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let input = get_input();
    let games = parse_input(&input);

    let part1_res = calc_part1(&games);
    println!("part1 res {part1_res:?}");

    let part2_res = calc_part2(&games);
    println!("part1 res {part2_res:?}");
}

fn get_input() -> Vec<String> {
    io::stdin().lines()
        .map(|line| line.expect("Unable to read line"))
        .collect()
}

fn parse_input(input: &Vec<String>) -> Vec<Game> {
    let parser = Regex::new(r"^Game (\d+): (.+)$").unwrap();
    let round_parser = Regex::new(r"(\d+)\s*(blue|red|green)").unwrap();
    input
        .iter()
        .map(|line| {
            let (_, [game_id_raw, rounds_raw]) = parser.captures(line).unwrap().extract();

            let game_id = game_id_raw.parse::<u32>().expect("Failed to parse game_id");

            let rounds = rounds_raw.split("; ")
                .map(|round_raw| {
                    let mut colors: HashMap<String, u32> = HashMap::new();

                    for cap in round_parser.captures_iter(round_raw) {
                        let value = cap[1].parse::<u32>().expect("Failed to parse balls qty");
                        colors.insert(cap[2].to_string(), value);
                    }

                    RoundRetrieve {
                        red: *colors.get("red").unwrap_or(&0),
                        green: *colors.get("green").unwrap_or(&0),
                        blue: *colors.get("blue").unwrap_or(&0),
                    }
                }
                )
                .collect()
                ;

            Game { id: game_id, rounds }
        })
        .collect()
}

fn calc_part1(games: &Vec<Game>) -> u32 {
    let bag = RoundRetrieve { red: 12, green: 13, blue: 14 };

    games
        .iter()
        .filter(|game| game.rounds
            .iter()
            .all(|round| round.red <= bag.red && round.green <= bag.green && round.blue <= bag.blue))
        .map(|game| game.id)
        .sum()
}

fn calc_part2(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .map(|game| {
            let possible_red = game.rounds
                .iter()
                .max_by(|r1, r2| r1.red.cmp(&r2.red))
                .map(|r| r.red)
                .unwrap();
            let possible_green = game.rounds
                .iter()
                .max_by(|r1, r2| r1.green.cmp(&r2.green))
                .map(|r| r.green)
                .unwrap();
            let possible_blue = game.rounds
                .iter()
                .max_by(|r1, r2| r1.blue.cmp(&r2.blue))
                .map(|r| r.blue)
                .unwrap();

            possible_red * possible_green * possible_blue
        })
        .sum()
}