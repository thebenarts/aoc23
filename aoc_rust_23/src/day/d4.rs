use crate::aoc_reader;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    card_numbers: HashSet<u32>,
}

fn read_input() -> Vec<Card> {
    let input = aoc_reader::read_day_input(4);
    let mut all_cards: Vec<Card> = Vec::new();

    for mut line in input.lines() {
        let mut card = Card {
            winning_numbers: HashSet::new(),
            card_numbers: HashSet::new(),
        };
        let mut line = line.split(":");
        let mut line = line.nth(1).unwrap();
        let mut nums_split = line.split("|");
        let winning_nums = nums_split.next().unwrap();
        let mut winning_nums = winning_nums.split_whitespace();
        for num in winning_nums {
            card.winning_numbers.insert(num.parse::<u32>().unwrap());
        }
        let card_nums = nums_split.next().unwrap();
        let mut card_nums = card_nums.split_whitespace();
        for num in card_nums {
            card.card_numbers.insert(num.parse::<u32>().unwrap());
        }

        all_cards.push(card);
    }

    return all_cards;
}

impl Card {
    fn Calculate_Part1(&self) -> u32 {
        let mut n = 0;
        for num in &self.card_numbers {
            if self.winning_numbers.contains(num) {
                n += 1;
            }
        }

        if n > 0 {
            n -= 1;
            n = 1 << n;
        }
        return n;
    }

    fn get_number_of_matches(&self) -> u32 {
        let mut n = 0;
        for num in &self.card_numbers {
            if self.winning_numbers.contains(num) {
                n += 1;
            }
        }

        return n;
    }
}

pub fn part1() {
    let cards = read_input();
    let mut result = 0;
    for card in &cards {
        println!("{:?}", card);
        let n = card.Calculate_Part1();
        println!("{}", n);
        result += n;
    }

    println!("result : {}", result);
}

pub fn part2() {
    let cards = read_input();
    let mut result = 0;
    let mut map: HashMap<u32, u32> = HashMap::new();
    for (i, card) in cards.iter().enumerate() {
        map.insert(i as u32, 1);
    }

    for (i, card) in cards.iter().enumerate() {
        let n_of_cards = map.get(&(i as u32)).unwrap().clone();
        result += n_of_cards;
        let mut n_matches = card.get_number_of_matches();
        // println!(
        //     "card {} : number of cards{} ---- number of matches{}",
        //     i, n_of_cards, n_matches
        // );

        for matching_num in 0..n_matches {
            println!("{i} ---- {}", matching_num);
            let iterator: u32 = i as u32 + matching_num as u32 + 1;
            if let Some(amount_of_card_x) = map.get_mut(&iterator) {
                *amount_of_card_x = *amount_of_card_x + n_of_cards;
            }
        }
    }

    // println!("{:?}", map);
    println!("result : {}", result);
}
