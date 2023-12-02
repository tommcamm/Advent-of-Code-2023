use std::io::{self, BufRead};

struct Game {
    id: i32,
    rounds: Vec<(i32, i32, i32)>, // Each tuple represents (red, green, blue)
}

fn main() {
    let stdin = io::stdin();
    let mut games = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() {
            break;
        }
        let game = parse_game(&line);
        games.push(game);
    }

    let sum_ids = games.iter()
        .filter(|game| is_game_possible(game))
        .map(|game| game.id)
        .sum::<i32>();

    let sum_powers = games.iter()
        .map(min_cubes_power)
        .sum::<i32>();

    println!("[Part one] Sum of IDs of possible games: {}", sum_ids);
    println!("[Part two] Sum of the power of minimum sets: {}", sum_powers);
}
fn parse_game(line: &str) -> Game {
    let parts: Vec<&str> = line.split(": ").collect();
    let id = parts[0].split(" ").nth(1).unwrap().parse().unwrap();
    let rounds = parts[1]
        .split("; ")
        .map(|s| {
            let counts = s.split(", ").fold((0, 0, 0), |mut acc, x| {
                let parts: Vec<&str> = x.split_whitespace().collect();
                let count = parts[0].parse::<i32>().unwrap();
                match parts[1] {
                    "red" => acc.0 += count,
                    "green" => acc.1 += count,
                    "blue" => acc.2 += count,
                    _ => {}
                }
                acc
            });
            counts
        })
        .collect();

    Game { id, rounds }
}

fn is_game_possible(game: &Game) -> bool {
    game.rounds.iter().all(|&(red, green, blue)| {
        red <= 12 && green <= 13 && blue <= 14
    })
}

fn min_cubes_power(game: &Game) -> i32 {
    let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);

    for &(red, green, blue) in &game.rounds {
        min_red = min_red.max(red);
        min_green = min_green.max(green);
        min_blue = min_blue.max(blue);
    }

    min_red * min_green * min_blue
}
