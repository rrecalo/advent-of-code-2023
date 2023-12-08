use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Hand {
   pub cards: Vec<char>,
   pub bid: u32,
}

impl Hand {
    
    pub fn best_play(&self) -> u32 {

        let char_set: HashSet<_> = self.cards.iter().cloned().collect();
        if char_set.len() == 1 { return 7;}
        let mut char_map = HashMap::new();

        if char_set.len() > 1 {
            for ch in self.cards.iter(){
                let counter = char_map.entry(ch).or_insert(0);
                *counter +=1;
            }
        }

        let mut values: Vec<u8> = vec![0, 0, 0, 0];
        for &count in char_map.values(){
            if count == 4 {
                values[0] = values[0] + 1;
                return 6;
            }
            else if count == 3 {
                values[1] = values[1] + 1;
            }
            else if count == 2 {
                values[2] = values[2] + 1;
            }
        }

        //3 of one, 2 of one => Full House
        if values[1] == 1 && values[2] == 1{
            return 5;
        }
        //3 of one => Three of a kind
        if values[1] == 1{
            return 4;
        }
        //2 pairs of two => Two pair
        else if values[2] == 2{
            return 3;
        }
        else if values[2] == 1{
            return 2;
        }
        else{
            return 1; 
        }

        
    }

}


fn compare_hands(a: &Hand, b: &Hand) -> std::cmp::Ordering{
    let order: Vec<char> = vec!['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    
    let a_rank = a.best_play();
    let b_rank = b.best_play();

    
    if a_rank == b_rank {
        loop {
            for i in 0..=4{

                let a_card_index = order.iter().position(|x| x == a.cards.get(i).unwrap()).unwrap() as u32;
                let b_card_index = order.iter().position(|x| x == b.cards.get(i).unwrap()).unwrap() as u32;
                
                if a_card_index != b_card_index { 
                    return a_card_index.cmp(&b_card_index);
                }

            }
            return std::cmp::Ordering::Equal;
        }
    }
    
    else{
        return a_rank.cmp(&b_rank);
    }

}

fn main() {

    let reader = BufReader::new(File::open("data.txt").unwrap());

    let mut hands: Vec<Hand> = Vec::new();
    
    for line in reader.lines(){
        let line = line.unwrap();
        let (cards, bid) = line.split_once(" ").unwrap();
        
        let card_vec: Vec<char> = cards.chars().collect::<Vec<char>>();
        
        let hand = Hand {cards: card_vec, bid: bid.parse().unwrap()};
    
        hands.push(hand);
    
    }

    hands.sort_by(compare_hands);
    let mut winnings: u32 = 0;
    for (index, hand) in hands.iter().enumerate() {
        winnings = winnings + (hand.bid * ((index as u32) + 1));
        
        /* 
        for card in &hand.cards{
            print!("{}", card);
        }
        print!(" {0} * {1} ", hand.bid, index+1);

        println!("");
        */
        
    }
    
    println!("{}", winnings);

}
