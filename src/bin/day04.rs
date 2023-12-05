use std::io;
use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    id: usize,
    winning: HashSet<u32>,
    my_own: HashSet<u32>,
}

fn main() {
    let input = get_input();
    let cards = parse_input(&input);

    let part1_res = calc_part1(&cards);
    println!("part1 res {part1_res:?}");
    //
    // let part2_res = calc_part2(&input);
    // println!("part1 res {part2_res:?}");
}

fn get_input() -> Vec<String> {
    io::stdin()
        .lines()
        .map(|line| line.expect("Unable to read line"))
        .collect()
}

fn parse_input(input: &Vec<String>) -> Vec<Card> {
    input
        .iter()
        .map(|line|
            line.split_once(": ")
                .map(|(card_part, numbers_part)| {
                    let card_id = card_part.split_whitespace()
                        .nth(1)
                        .and_then(|id_str| id_str.parse::<usize>().ok())
                        .expect("Bad card id");
                    
                    let (winning, my_own): (HashSet<u32>, HashSet<u32>) = numbers_part.split_once(" | ")
                        .map(|(winning_str, my_own_str)|
                            (
                                winning_str.split(" ")
                                    .map(|n| n.parse::<u32>()?)
                                    .collect()
                                ,
                                my_own_str.split(" ")
                                    .map(|n| n.parse::<u32>()?)
                                    .collect()
                            )
                        )
                        .unwrap();

                    Card { id: card_id, winning, my_own }
                })
                .unwrap()
        )
        .collect()
}

fn calc_part1(cards: &Vec<Card>) -> u32 {
    println!("{:?}", cards);
    todo!()
}

fn calc_part2(input: &Vec<String>) -> u32 {
    todo!()
}
