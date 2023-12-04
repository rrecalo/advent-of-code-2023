use std::char;
use std::io::{BufRead, BufReader};
use std::fs::File;

struct Card {
    numbers: Vec<u32>
}


fn get_total_cards(path_name: &str) -> u32 {

    let mut points = 0;

    let file = File::open(path_name).expect("File not found at!");

    let reader = BufReader::new(file);

    let mut card_counts: Vec<u32> = Vec::new();

    let mut card_total: u32 = 0;

    for (card_num, line) in reader.lines().enumerate(){
        let mut multiples: u32 =1;
        if card_counts.len() > 0 {
            multiples += *card_counts.get(0).unwrap();
            card_counts.remove(0);
        }
        card_total+= multiples;

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

        let mut card_points: u32 = 0;
        let mut match_count: u32 = 0;
        
        for num in card {
            if winning_nums.contains(&num) {
                match card_points {
                    0 => {
                        card_points=1;
                        match_count+=1;
                    },

                    1..=u32::MAX => {
                        card_points *= 2;
                        match_count+=1;
                    },
                };
            };

        };

        for i in 0..std::cmp::min(match_count as usize, card_counts.len()){
            card_counts[i] += 1*multiples;
        };
        
        //from length -> match_count, push 1 to the vector
        for _ in card_counts.len()..match_count as usize {
            card_counts.push(1*multiples);
        }

    }
    

    return card_total;
}

fn main() {

    let points = get_total_cards("data.txt");
    println!("card : {}", points);    

}
