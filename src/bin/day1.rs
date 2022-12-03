use std::fs;

fn main() {

// Initialize 

let s: String = get_input();

let elves_raw: Vec<&str> = s.split("\n\n").collect();

let elves: Vec<Elf> = elves_raw.iter().map(|x| Constructor::new(x.to_string()) ).collect();

// Part 1

let winner: &Elf = elves.iter().reduce(|most, elf| if elf.sum > most.sum { elf } else { most } ).unwrap();

println!("{}", winner.sum);

// Part 2

let mut top_three: [i32; 3] = [0;3];

for elf in elves {
    if elf.sum > top_three[0] {
        top_three[0] = elf.sum;
        top_three.sort();
    }
}

println!("{}", top_three.iter().sum::<i32>())

}

struct Elf {
    sum: i32
}

trait Constructor {
    fn new(raw: String ) -> Self;
}

impl Constructor for Elf {
    fn new(raw: String) -> Elf {
        let food: Vec<i32> = raw.split('\n').map(|x| i32::from_str_radix(x,10).unwrap()).collect();
        let sum: i32 = food.iter().sum();
        Elf {  sum: sum }
    }
}


fn get_input() -> String {
    let path = "resources/day1input.txt";

//    String::from_utf8().unwrap()
    String::from_utf8(fs::read(path).unwrap()).unwrap()
}