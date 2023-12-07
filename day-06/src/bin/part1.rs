use std::io::{BufRead, BufReader, Read};
use std::fs::File;


fn main() {
    
    let mut reader = BufReader::new(File::open("data.txt").unwrap());

    let mut buf: String = String::new();

    let _ = reader.read_to_string(&mut buf);

    let (_, times_str) = buf.lines().next().unwrap().split_once(":").unwrap();
    let (_, distances_str) = buf.lines().last().unwrap().split_once(":").unwrap();

    let time_str = times_str.trim();

    let split_times : Vec<&str> = time_str.split_whitespace().collect();

    let times: Vec<u32> = split_times.into_iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    let distances_str = distances_str.trim();

    let split_d: Vec<&str> = distances_str.split_whitespace().collect();

    let distances: Vec<u32> = split_d.into_iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut records: Vec<u32> = Vec::new();
    for (race_index, time) in times.iter().enumerate(){
    let mut possibilities: Vec<u32> = Vec::new();
    for i in 1..=time-1{
         let traveled: u32 = (time - i) * i;
         if traveled > distances[race_index] {
            possibilities.push(i);
         }
    }
        records.push(possibilities.len() as u32);

    }

    println!("{}", records.iter().fold(1, |acc, &x| acc * x));   

}
