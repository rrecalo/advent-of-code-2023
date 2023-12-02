use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Game {
   pub number: u32,
   pub sets: Vec<Set>
}

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

fn find_possible_games(games: Vec<Game>, cubes: Set) -> Vec<u32> {

    let mut game_ids: Vec<u32> = Vec::new();
    
    for game in games{
        
        let mut possible: bool = true;
            
        for set in game.sets{
            match set {
                Set { red, ..} if red > cubes.red => {
                    possible = false;
                },
                Set { green, ..} if green > cubes.green => {
                    possible = false;
                },
                Set { blue, ..} if blue > cubes.blue => {
                    possible = false;
                },
                _ => {}
            }
        }
        if possible {
            game_ids.push(game.number);
        }

    }

    return game_ids;

}

fn main() {
    let file_path = "../../input.txt";
    let games: Vec<Game> = read_games(file_path);

    let cubes: Set = Set {red:12, green:13, blue:14};
    let game_ids: Vec<u32> = find_possible_games(games, cubes);
    
    println!("{}", game_ids.iter().sum::<u32>());

}
