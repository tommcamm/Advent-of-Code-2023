use std::io;
use std::io::BufRead;

fn main() {

    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(l) => lines.push(l),
            Err(e) => {
                eprintln!("Error reading line {}", e);
                break;
            }
        }
    }

    part_one(&lines);
    part_two(&lines);
}

fn part_one(lines : &[String]) {

    let mut total = 0;

    lines.iter().for_each(|line| {
        let mut digits = String::new();

        for char in line.chars() {
            if char.is_numeric() {
                digits.push(char);
                break;
            }
        }

        for char in line.chars().rev() {
            if char.is_numeric() {
                digits.push(char);
                break;
            }
        }

        if !digits.is_empty() {
            total += digits.parse::<i32>().unwrap();
        }

    });

    println!("[Part one] The sum is {}", total);
}

fn part_two(lines : &[String]) {
    let mut total = 0;

    lines.iter().for_each(|line| {
        let mut digits = String::new();

        // Let's find first digit
        let converted = string_converter(line);
        for char in converted.chars() {
            if char.is_numeric() {
                digits.push(char);
                break;
            }
        }

        // Let's find first digit backwards
        println!("Backward search");
        let converted_rev = string_converter_rev(line);
        println!("converted rev: {}", converted_rev);
        for char in converted_rev.chars().rev() {
            if char.is_numeric() {
                digits.push(char);
                break;
            }
        }

        if !digits.is_empty() {
            println!("Found digits are -> {:?}", &digits);
            total += digits.parse::<i32>().unwrap();
        }

    });

    println!("[Part two] The sum is {}", total);

}

fn string_converter(line: &str) -> String {
    let num_words = [("one", '1'), ("two", '2'), ("three", '3'),
        ("four", '4'), ("five", '5'), ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9')];
    let mut result = String::new();
    let mut temp = String::new();

    for ch in line.chars() {
        if ch.is_alphabetic() {
            temp.push(ch);
            if let Some(&(word, digit)) = num_words.iter().find(|&&(word, _)| temp.ends_with(word)) {
                let word_len = word.len();
                temp.truncate(temp.len() - word_len);
                temp.push(digit);
            }
        } else {
            result.push_str(&temp);
            temp.clear();
            result.push(ch);
        }
    }

    if !temp.is_empty() {
        result.push_str(&temp);
    }

    result
}


fn string_converter_rev(line: &str) -> String {
    let num_words = [("nine", '9'), ("eight", '8'), ("seven", '7'), ("six", '6'),
        ("five", '5'), ("four", '4'), ("three", '3'), ("two", '2'), ("one", '1')];
    let mut result = String::new();
    let mut temp = String::new();

    for ch in line.chars().rev() {
        if ch.is_alphabetic() {
            temp.insert(0, ch);
            if let Some(&(word, digit)) = num_words.iter().find(|&&(word, _)| temp.starts_with(word)) {
                result.push(digit);
                temp = temp.split_at(word.len()).1.to_string();
            }
        } else {
            if !temp.is_empty() {
                result.extend(temp.chars().rev());
                temp.clear();
            }
            result.push(ch);
        }
    }

    if !temp.is_empty() {
        result.extend(temp.chars().rev());
    }

    result.chars().rev().collect()
}

