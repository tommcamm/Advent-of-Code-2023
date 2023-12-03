use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let schematic: Vec<Vec<char>> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let sum_part_numbers = sum_part_numbers(&schematic);
    let sum_gear_ratios = sum_gear_ratios(&schematic);

    println!("[Part one] Sum of all part numbers: {}", sum_part_numbers);
    println!("[Part two] Sum of gear ratios: {}", sum_gear_ratios);
}

// Part one
fn sum_part_numbers(schematic: &[Vec<char>]) -> i32 {
    let mut sum = 0;
    let mut counted_positions = HashSet::new();

    for (y, row) in schematic.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if is_symbol(cell) {
                for (adj_x, adj_y) in get_adjacent_positions(x, y, schematic) {
                    if !counted_positions.contains(&(adj_x, adj_y)) && schematic[adj_y][adj_x]
                        .is_ascii_digit() {
                        if let Some(number) = extract_number(adj_x, adj_y, schematic) {
                            sum += number;
                            for pos in number_positions(adj_x, adj_y,
                                                        &number.to_string(), schematic) {
                                counted_positions.insert(pos);
                            }
                        }
                    }
                }
            }
        }
    }
    //counted_positions.iter().for_each(|pos| println!("Pos: {:?}", pos));
    sum
}

// Part two
fn sum_gear_ratios(schematic: &[Vec<char>]) -> i32 {
    let mut sum = 0;
    let mut counted_positions = HashSet::new();

    for (y, row) in schematic.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '*' && get_adjacent_positions(x, y, schematic).len() > 1 {
                let mut temp_mult = 1;
                let mut counter = 0 ;
                for (adj_x, adj_y) in get_adjacent_positions(x, y, schematic) {
                    if !counted_positions.contains(&(adj_x, adj_y)) && schematic[adj_y][adj_x]
                        .is_ascii_digit() {
                        if let Some(number) = extract_number(adj_x, adj_y, schematic) {
                            temp_mult *= number;
                            counter += 1;
                            for pos in number_positions(adj_x, adj_y,
                                                        &number.to_string(), schematic) {
                                counted_positions.insert(pos);
                            }
                        }
                    }
                }

                // Giant work-around, should be changed to something that avoid useless work above.
                if counter == 2 {
                    sum += temp_mult;
                }
            }
        }
    }
    //counted_positions.iter().for_each(|pos| println!("Pos: {:?}", pos));
    sum
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.' && !c.is_ascii_control()
}

fn get_adjacent_positions(x: usize, y: usize, grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut adjacent_positions = Vec::new();
    let directions = [
        (-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1),
    ];

    for (dx, dy) in directions {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;
        if new_x >= 0 && new_y >= 0 && new_y < grid.len() as i32 && new_x < grid[new_y as usize]
            .len() as i32 {
            adjacent_positions.push((new_x as usize, new_y as usize));
        }
    }

    adjacent_positions
}

fn extract_number(x: usize, y: usize, grid: &[Vec<char>]) -> Option<i32> {
    let mut number_str = String::new();

    // We scroll left to consider when we are not in the first part of the string
    let mut current_x = x;
    while current_x > 0 && grid[y][current_x - 1].is_ascii_digit() {
        current_x -= 1;
    }

    while current_x < grid[y].len() && grid[y][current_x].is_ascii_digit() {
        number_str.push(grid[y][current_x]);
        current_x += 1;
    }

    number_str.parse::<i32>().ok()
}


fn number_positions(x: usize, y: usize, number_str: &str,
                    grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    // left scrolling - mannaggggg
    let mut current_x = x;
    while current_x > 0 && grid[y][current_x - 1].is_ascii_digit() {
        current_x -= 1;
    }
    //println!("actual x: x->{}, y->{}", x, y);

    for offset in 0..number_str.len() {
        if current_x + offset < grid[y].len() {
            positions.push((current_x + offset, y));
        }
    }
    positions
}

