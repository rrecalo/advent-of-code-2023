use std::io::{BufReader, Read};
use std::fs::File;


fn main() {
    
    let mut reader = BufReader::new(File::open("data.txt").unwrap());

    let mut buf: String = String::new();

    let _ = reader.read_to_string(&mut buf);

    let (_, times_str) = buf.lines().next().unwrap().split_once(":").unwrap();
    let (_, distances_str) = buf.lines().last().unwrap().split_once(":").unwrap();

    let time_str = times_str.trim();

    //let split_times : Vec<&str> = time_str.split_whitespace().collect();

    let time: u64 = time_str.parse().unwrap();

    let distances_str = distances_str.trim();

    //let split_d: Vec<&str> = distances_str.split_whitespace().collect();

    let distance: u64 = distances_str.parse().unwrap();

    let mut records: Vec<u64> = Vec::new();
    let mut possibilities: Vec<u64> = Vec::new();
    for i in 1..=time-1{
         let traveled: u64 = (time - i) * i;
         if traveled > distance {
            possibilities.push(i);
         }
    }
        records.push(possibilities.len() as u64);


    println!("{}", records.iter().fold(1, |acc, &x| acc * x));   

}
