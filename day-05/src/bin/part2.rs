use std::fs::File;
use std::io::{BufRead, BufReader};
use rayon::prelude::*;

#[derive(Clone, Copy)]
struct SeedMap{
    pub source: u64,
    pub destination: u64,
    pub range: u64,
}   

fn main() {
    
    let reader = BufReader::new(File::open("data.txt").unwrap());
    let mut seed_maps: Vec<Vec<SeedMap>> = Vec::new();
    let mut seeds: Vec<(u64, u64)> = Vec::new();
    let mut current_map: Vec<SeedMap> = Vec::new();

    let all_lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    // Get the usize representing the number of lines
    let number_of_lines = all_lines.len();

    for (row, line) in all_lines.iter().enumerate(){
        let line = line;

        if line.starts_with("seeds:") {
            let slice = &line[6..].trim();

            let mut digit_str: String = String::new();
            let mut seed_pair: (u64, u64) = (0, 0);
            for (index, char) in slice.chars().enumerate() {
                match char {
                    ch if ch.is_digit(10) => {
                        digit_str+= &ch.to_string();
                        if index == slice.len()-1 {
                            seed_pair.1 = digit_str.parse().unwrap();
                            seeds.push(seed_pair);
                            digit_str = String::new();
                        }
                    },
                    _ => {
                        if seed_pair.0 == 0 {
                            seed_pair.0 = digit_str.parse().unwrap();
                            digit_str = String::new();
                        }
                        else if seed_pair.1 == 0 {
                            seed_pair.1 = digit_str.parse().unwrap();
                            seeds.push(seed_pair);
                            seed_pair = (0, 0);
                            digit_str = String::new();
                        }
                        else if seed_pair.0 != 0 && seed_pair.1 != 0{
                            seeds.push(seed_pair);
                            seed_pair = (0, 0);
                        }
                    },
                };
            }
            continue;
            /*
            for s in seeds{
                println!("{}", s);
            }
            */
        };
        let mut digit_str = String::new();
        let mut map_details: Vec<u64> = Vec::new();
        for (index, char) in line.chars().enumerate(){
            match char {
                ch if ch.is_digit(10) => {
                    digit_str+= &ch.to_string();

                    if index == line.len() - 1 {
                        map_details.push(digit_str.parse().unwrap());
                        digit_str = String::new();
                    };
                },
                _=>{
                    if digit_str.len() > 0 {
                        map_details.push(digit_str.parse().unwrap());
                        digit_str = String::new();
                    }
                },
            }
        };

        if line.is_empty() {
            if current_map.len() > 0 {
                seed_maps.push(current_map.clone());
                current_map = Vec::new();
            };
        };

        if map_details.len() > 0 {
            current_map.push(SeedMap {source: *map_details.get(1).unwrap(),
            destination: *map_details.get(0).unwrap(), range: *map_details.get(2).unwrap()});
            if row == number_of_lines - 1 {
                seed_maps.push(current_map.clone());
            }   
        };
        
    };
    

    let mut loc_numbers: Vec<u64> = Vec::new();
    
    for pair in &seeds{
        println!("{0}, {1}", pair.0, pair.1);
    }
    

     
    for pair in seeds{
        let seed_range = pair.0..pair.0+pair.1;
        let possible_nums = seed_range.collect::<Vec<_>>().par_iter().map(|num| {
        let mut new_num: u64 = *num;
        for s_map in &seed_maps{
            'inner : for seed_map in s_map.iter(){

                if new_num >= seed_map.source && new_num < seed_map.source+seed_map.range {
                    let diff = new_num - seed_map.source;
                    new_num = seed_map.destination + diff;
                    break 'inner;
                }
            }
        }
        return new_num;
        }).collect::<Vec<u64>>();

        for n in possible_nums{
            loc_numbers.push(n);
        }
    }
  
    let mut lowest = std::u64::MAX;
    for n in loc_numbers{
        lowest = u64::min(n, lowest);
    }
   
    
    println!("{}", lowest);
    

}
