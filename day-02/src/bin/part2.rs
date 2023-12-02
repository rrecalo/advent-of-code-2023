use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
pub struct Game {
   pub number: u32,
   pub sets: Vec<Set>
}

#[derive(Clone)]
pub struct Set {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

fn read_games(path: &str) -> Vec<Game> {
   
    let mut games: Vec<Game> = Vec::new();
    let game_file = File::open(path).unwrap();
    
    let reader = BufReader::new(game_file);
    for line in reader.lines(){

        let line = line.unwrap();
        let (game_str, sets_str) = line.split_once(":").unwrap();
        let game_id = game_str.split_once(" ").unwrap().1.parse().expect("Invalid number to parse");
        
        let mut sets: Vec<Set> = Vec::new();
        for set in sets_str.split(";"){
            let draws = set.split(",");
            let mut set_drawn: Set = Set {red:0, green:0, blue:0};
            for draw in draws {
                let draw = draw.trim();
                let (number_drawn, color_drawn) = draw.split_once(" ").unwrap();
                match color_drawn{
                    "red" => set_drawn.red = number_drawn.parse::<u32>().unwrap(),
                    "green" => set_drawn.green = number_drawn.parse::<u32>().unwrap(),
                    "blue" => set_drawn.blue = number_drawn.parse::<u32>().unwrap(),
                    _ => {},
                }
            }
            sets.push(set_drawn);
        }

        games.push( Game{number: game_id, sets});
        
    }
    
    return games;
}

fn find_sum_of_powers(games: Vec<Game>) -> u32 {
    
    let mut powers: Vec<u32> = Vec::new(); 

    for game in games {
        let mut min_set: Set = game.sets.clone().first().unwrap().clone();
        for set in game.sets {
            if set.red > min_set.red {
                    min_set.red = set.red;
            };
            if set.green > min_set.green {
                min_set.green = set.green;
            };
            if set.blue > min_set.blue  {
                min_set.blue = set.blue;
            };
        };
        match min_set {
            Set { red, ..} if red == 0 => { min_set.red = 1},
            Set { green, ..} if green == 0 => { min_set.green = 1},
            Set { blue, ..} if blue == 0 => {min_set.blue = 1},
            _ => {},
        };
        let power_of_set = min_set.red*min_set.green*min_set.blue;
        powers.push(power_of_set);
    };

    return powers.iter().sum::<u32>()
}



fn main() {
    let file_path = "./input.txt";
    let games: Vec<Game> = read_games(file_path);
       
    let sum = find_sum_of_powers(games);
    println!("{}", sum);

}
