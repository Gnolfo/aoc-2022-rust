use std::fs;

fn main() {

// Initialize 

let s: String = get_input();

let rounds_raw: Vec<&str> = s.split("\n").collect();

let rounds: Vec<Round> = rounds_raw.iter().map(|x| Constructor::new(x.to_string()) ).collect();

// Part 1

let mut score:u32 = rounds.iter().fold(0u32, |acc, round| round.get_score(true) as u32 + acc );

println!("{}", score);

// Part 2

score = rounds.iter().fold(0u32, |acc, round| round.get_score(false) as u32 + acc );

println!("{}", score);

}

struct Round {
    opponent:u8,
    choice1:u8,
    choice2:u8
}

trait Constructor {
    fn new(raw: String ) -> Self;
}

impl Constructor for Round {
    fn new(mut raw: String) -> Round {

        let key : u8;
        let opponent : u8;
        unsafe {
            let v = raw.as_mut_vec();
            key = v[2] - ('X' as u8);
            opponent = v[0] - ('A' as u8);
        }
        // For choice 2...
        // 0 => 2 more than opponent, 1 => same as opp, 2 => 1 more
        // 0:2, 1:0, 2:1 => (key+2)%3 added to opponent and modded by 3

        Round {opponent:opponent, choice1: key, choice2: (opponent + ((key+2)%3))%3}
    }
}

impl Round {

    fn get_score(&self, first:bool) -> u8 {
        // if R=0, P=1, S=2 ...
        // score = 3 *
        //      ((3 + me - opponent) % 3 + 1 % 3 )
        let choice: u8;
        if first {
            choice = self.choice1;
        } else {
            choice = self.choice2;
        }
        return (1+choice) + 3*((1+(3+choice - self.opponent)%3)%3);
    }
}

fn get_input() -> String {
    let path = "resources/day2input.txt";


    String::from_utf8(fs::read(path).unwrap()).unwrap()
}