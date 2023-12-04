use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Card {
    winning_numbers: HashSet<i32>,
    extracted_numbers: HashSet<i32>,
    card_no: i32,
}

impl Card {
    fn calculate_p1_score(&self) -> i32 {
        let mut score = 0;
        self.extracted_numbers.iter()
            .filter(|number| self.winning_numbers.contains(number))
            .for_each(|_| {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            });

        //println!("Card {}, score -> {}", self.card_no, score);
        score
    }

    fn get_matching_numbers(&self) -> i32 {
        self.extracted_numbers
            .iter().filter(|number| self.winning_numbers.contains(number)).count() as i32
    }

}

fn main() {
    let stdin = io::stdin();
    let card_sets : Vec<Card> = stdin.lock().lines()
        .map(|line| parse_card(line.unwrap())).collect();

    println!("[Part one] The sum of points is {}", p1_sum(&card_sets));
    println!("[Part two] The sum of the cards is {}", p2_sum(&card_sets));
}

fn p1_sum(cards: &[Card]) -> i32{
    let mut sum = 0;
    cards.iter().for_each(|card| {sum += card.calculate_p1_score()});
    sum
}

fn p2_sum(cards: &[Card]) -> i32 {
    let points_vec :Vec<i32> = cards.iter()
        .map(|card| card.get_matching_numbers()).collect();

    let mut res_vec = vec![1; points_vec.len()];

    for (i, num) in points_vec.iter().enumerate() {
        for _ in 0..*res_vec.get(i).unwrap() {
            for delta in 1..(*num + 1) {
                if delta <= points_vec.len() as i32 {
                    res_vec[(delta + (i as i32)) as usize] += 1;
                }

            }
        }
    }

    res_vec.iter().sum::<i32>() // sum of the vector of cards
}

fn parse_card(line: String) -> Card {
    let id_regex = Regex::new(r"\d+").unwrap();

    let parts :Vec<&str> = line.split(": ").collect();

    // I'm lazy, I get the ID with regex
    let id_str = id_regex.captures(parts[0])
        .and_then(|cap| cap.get(0))
        .map_or("", |m| m.as_str());
    let id = i32::from_str(id_str).unwrap();

    let winning_numbers = parts[1].split(" | ").next().unwrap()
        .split_ascii_whitespace().map(|num| num.parse::<i32>().unwrap()).collect();

    let extracted_numbers = parts[1].split(" | ")
        .nth(1).unwrap()
        .split_ascii_whitespace().map(|num| num.parse::<i32>().unwrap()).collect();

    Card {
        winning_numbers,
        extracted_numbers,
        card_no: id,
    }
}

