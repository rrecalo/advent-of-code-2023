use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    
    part1("calibration_doc.txt");
    println!("{}", part1("data.txt")); 
    Ok(())
}

fn part1(file_name: &str) -> u32 {

    let file = File::open(file_name).unwrap();

    let reader = io::BufReader::new(file);

    let mut total: u32 = 0;

    for line in reader.lines(){
    
        let line = line.unwrap();
        let mut nums: Vec<u32> = Vec::new();

        for byte in line.into_bytes().iter(){
           
            let ch = *byte as char;

            if ch.is_digit(10){
                nums.push(ch.to_digit(10).unwrap());
            }   

        }

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
        let result = part1("calibration_doc.txt");
        assert_eq!(result, 142);
    }

}
