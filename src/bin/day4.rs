use std::fs;
use std::ops::Range;

fn main() {

// Initialize 

let s: String = get_input();

let pairs_raw: Vec<&str> = s.split("\n").collect();

let pairs: Vec<Pair> = pairs_raw.iter().map(|x| Constructor::new(x.to_string()) ).collect();

// Part 1
let mut score:u32 = pairs.iter().filter(|x| x.overlaps_fully() ).count() as u32;
//rounds.iter().fold(0u32, |acc, round| round.get_score(true) as u32 + acc );

println!("{}", score);

// Part 2

score = pairs.iter().filter(|x| x.overlaps_some() ).count() as u32;

println!("{}", score);

}

#[derive(Copy,Clone,Debug)]
struct Pair {
    // tuple is adjusted so it can be converted straight into a Range and deal with exclusive end
    left: (i32, i32),
    right: (i32, i32)
}




impl Pair {

    fn get_left(self) -> Range<i32> {
        Range { start: self.left.0, end: self.left.1}
    }

    fn get_right(self) -> Range<i32> {
        Range { start: self.right.0, end: self.right.1}
    }

    fn overlaps_fully(self) -> bool {
        self.get_left().all(|x| self.get_right().contains(&x)) || 
        self.get_right().all(|y| self.get_left().contains(&y))
    }

    fn overlaps_some(self) -> bool {
        self.get_left().any(|x| self.get_right().contains(&x))
    }
    
}

trait Constructor {
    fn new(raw: String ) -> Self;
}

impl Constructor for Pair {
    fn new(raw: String) -> Pair {

        let pair_raw: Vec<&str> = raw.split(',').collect();
        // there's certainly a cleaner way to do this but I'm still fighting with navigating types/borrowing in rust
        let left_raw: Vec<i32> = pair_raw[0].split('-').map(|x| x.parse::<i32>().unwrap()).collect();
        let right_raw: Vec<i32> = pair_raw[1].split('-').map(|x| x.parse::<i32>().unwrap()).collect();

        Pair {  left: (left_raw[0], 1+left_raw[1]),
                right: (right_raw[0], 1+right_raw[1])}
 
    }
}
 

fn get_input() -> String {
    let path = "resources/day4input.txt";


    String::from_utf8(fs::read(path).unwrap()).unwrap()
}