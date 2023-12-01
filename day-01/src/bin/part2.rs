use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    
    println!("{}", part2("data.txt"));
    Ok(())
}

fn word_to_number(word: &str) -> Option<u32> {
    match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn get_numbers_in_line(line : String) -> Vec<u32> {
    let valid_nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let line = line.clone();
    let mut nums_found: Vec<u32> = Vec::new();
    let mut mod_line = line.clone();
    for i in 0..line.len(){
        valid_nums.iter().any(|&num| 
            {
                if line[i..].starts_with(num) {
                    let digit = word_to_number(num).unwrap().to_string();
                    mod_line = mod_line.replace(num, &(num[0..1].to_string()+digit.as_str()+&num.chars().last().unwrap().to_string() ));
                    true
                }
                else {
                    false
                }
            });
    }

    println!("{}", mod_line);

    for byte in mod_line.into_bytes().iter(){
           
            let ch = *byte as char;

            if ch.is_digit(10){
                nums_found.push(ch.to_digit(10).unwrap());
            }   

        }
    
    nums_found
}

fn part2(file_name: &str) -> u32 {

    let file = File::open(file_name).unwrap();

    let reader = io::BufReader::new(file);

    let mut total: u32 = 0;

    for line in reader.lines(){
    
        let line = line.unwrap();
        let nums = get_numbers_in_line(line);
        
        let value: u32 = (nums.first().unwrap() * 10) + nums.last().unwrap();
        total+=value;

    }
    return total;
}


#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn sample_data(){
        let result = part2("word_sample.txt");
        assert_eq!(result, 281);
    }

}
