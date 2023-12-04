use std::char;
use std::io::{BufRead, BufReader};
use std::fs::File;

struct Card {
    numbers: Vec<u32>
}


fn find_points(path_name: &str) -> u32{

    let mut points = 0;

    let file = File::open(path_name).expect("File not found at!");

    let reader = BufReader::new(file);

    for line in reader.lines(){
        
        let line = line.expect("No line found!");

        let (mut card_str, mut winning_nums_str) = line.split_once("|").unwrap();

        card_str = &card_str[card_str.find(":").unwrap()+1..card_str.len()].trim();

        winning_nums_str = &winning_nums_str.trim();

        let mut winning_nums: Vec<u32> = Vec::new();
        let mut digit: String = String::new();

        for (index, char) in winning_nums_str.chars().enumerate(){
            match char {
                ch if ch.is_digit(10) => {
                    digit+=&ch.to_string();
                    if index == winning_nums_str.len()-1{
                        winning_nums.push(digit.parse().expect("Invalid parse from 'digit' String"));
                        digit = String::new();
                    };
                },
                _ => {
                    if !digit.is_empty() {
                        winning_nums.push(digit.parse().expect("Invalid parse from 'digit' String"));
                        digit = String::new();
                    };
                    continue;
                },
            };
        };
        
        let mut card: Vec<u32> = Vec::new();

        for (index, char) in card_str.chars().enumerate(){
            match char {
                ch if ch.is_digit(10) => {
                    digit+=&ch.to_string();
                    if index == card_str.len()-1{
                        card.push(digit.parse().expect("Invalid parse from 'digit' String"));
                        digit = String::new();
                    };
                },
                _ => {
                    if !digit.is_empty() {
                        card.push(digit.parse().expect("Invalid parse from 'digit' String"));
                        digit = String::new();
                    };
                    continue;
                },
            };
        }; 

        let mut card_points = 0;
        for num in card {
            if winning_nums.contains(&num) {
                match card_points {
                    0 => {
                        card_points=1;
                    },

                    1..=u32::MAX => {
                        card_points *= 2;
                    },
                };
            };
        };
        points+=card_points;
    }
    

    return points;
}

fn main() {

    let points = find_points("data.txt");
    println!("points : {}", points);    

}
