use std::fs::File;
use std::io::{BufRead, BufReader};


struct Part{
    number: u32,
    row: usize,
    position: (usize, usize),
}

fn sum_of_part_numbers(schematic: Vec<Vec<char>>) -> u32 {

    let mut parts: Vec<Part> = Vec::new();
    for (row, line) in schematic.iter().enumerate() {
        
        let mut part_number_str: String = String::new();
        let (mut start_index, mut end_index): (usize,usize) = (0, 0);

        for (index, char) in line.iter().enumerate() {
            match char {
                ch if ch.is_digit(10) => {
                    if part_number_str.is_empty() {
                        start_index=index; 
                    };
                    part_number_str+=&ch.to_string();
                    if index == line.len() -1 {
                        end_index = index -1;
                        let part_number: u32 = part_number_str.parse().expect("Invalid u32!");
                        parts.push(Part{number: part_number, position:(start_index, end_index), row});
                        part_number_str = String::new();

                    };
                },
                _ => {
                    //println!("{}", char);
                    if !part_number_str.is_empty(){
                        end_index = index -1;
                        let part_number: u32 = part_number_str.parse().expect("Invalid u32!");
                        parts.push(Part{number: part_number, position:(start_index, end_index), row});
                        part_number_str = String::new();
                    };
                },
            };
        };

    };
    
    parts.retain(|part| is_valid_part_number(&schematic, part));

    let mut sum: u32 = 0;

    for part in parts {
        sum+=part.number;
    }

    return sum;
}

fn is_valid_part_number(schematic: &Vec<Vec<char>>, part: &Part) -> bool {

    let mut is_valid: bool = false;

    if part.row > 0 {
        let prev_row = schematic.get(part.row - 1).unwrap();
        let start = match part.position.0 {
            0 => 0,
            _ =>{ part.position.0-1},
        };

        let end = match part.position.1 {
            _ if part.position.1 == prev_row.len() => part.position.1,
            _ if part.position.1 > prev_row.len() => prev_row.len(),
            _ => part.position.1+2,
        };
        
        let prev_row_slice = &prev_row[start..end];
        for char in prev_row_slice {
            match char {
                ch if ch.is_digit(10) => {continue;},
                ch if ch.to_string() == "." => {continue;},
                _ => {is_valid = true},
            };
        };
    };

    if part.row < schematic.len()-1 {
        let next_row = schematic.get(part.row + 1).unwrap();
        let start = match part.position.0 {
            0 => 0,
            _ =>{ part.position.0-1},
        };

        let end = match part.position.1 {
            _ if part.position.1 == next_row.len() => part.position.1,
            _ if part.position.1 > next_row.len() => next_row.len(),
            _ => part.position.1+2,
        };        let next_row_slice = &next_row[start..end];
        for char in next_row_slice {
            match char {
                ch if ch.is_digit(10) => {continue;},
                ch if ch.to_string() == "." => {continue;},
                _ => {is_valid = true},
            };
        };
    };

    let part_row = schematic.get(part.row).unwrap();

    let start = match part.position.0 {
            0 => 0,
            _ =>{ part.position.0-1},
    };
    let end = match part.position.1 {
            _ if part.position.1 == schematic.get(part.row).unwrap().len() => part.position.1,
            _ => part.position.1+2,
    };    let part_row_slice = &part_row[start..end];

    for char in part_row_slice {
        match char {
            ch if ch.is_digit(10) => {continue;},
            ch if ch.to_string() == "." => {continue;},
            _ => {is_valid = true},
        }
    };

    return is_valid;
}

fn load_schematic(file_path: &str) -> Vec<Vec<char>> {

    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);
    
    let mut schematic: Vec<Vec<char>> = Vec::new();

    for line in reader.lines(){
        let line = line.unwrap();
        
        let mut schematic_line: Vec<char> = Vec::new();

        for char in line.chars(){
            schematic_line.push(char);
        }
        schematic.push(schematic_line);

    }

    return schematic;
}

fn main() {
    
    let schematic = load_schematic("data.txt"); 
    let total = sum_of_part_numbers(schematic);
    
    println!("total: {}", total);

}
