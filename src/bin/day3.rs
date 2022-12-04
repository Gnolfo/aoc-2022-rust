use std::fs;

fn main() {

// Initialize 

let s: String = get_input();

let rucksacks_raw: Vec<&str> = s.split("\n").collect();

let mut rucksacks: Vec<Rucksack> = rucksacks_raw.iter().map(|x| Constructor::new(x.to_string()) ).collect();

// Part 1

let mut score:u32 = rucksacks.iter().fold(0u32, |acc, rucksack| get_priority(rucksack.item_type) as u32 + acc );

println!("{}", score);

// Part 2
let mut group: Vec<Rucksack>;
score = 0;

while rucksacks.len() > 0 {
    group = rucksacks.drain(..3).collect();
    let badges: Vec<char> = get_common_types(&group[0].items, &group[1].items);
    let probably_dont_need_this:String = badges.iter().collect();
    let finalists: Vec<char> = get_common_types(&probably_dont_need_this, &group[2].items);
    score += get_priority(finalists[0]) as u32;
}

//score = rounds.iter().fold(0u32, |acc, round| round.get_score(false) as u32 + acc );

println!("{}", score);

}

fn get_common_types(needles: &String, haystack:&String) -> Vec<char> {
    needles.as_str().chars().filter(|&x| haystack.find(x) != None).collect()
}

fn get_priority(item_type: char) -> u8 {

    let value: u32;
    if item_type.is_ascii_lowercase() {
        value = (item_type as u32) - ('a' as u32) + 1;
    } else {
        value = (item_type as u32) - ('A' as u32) + 27;
    }

    value.try_into().unwrap()
}

struct Rucksack {
    item_type:char,
    items: String
}

trait Constructor {
    fn new(raw: String ) -> Self;
}

impl Constructor for Rucksack {
    fn new(raw: String) -> Rucksack {
        let (first,second) = raw.split_at((raw.len()/2) as usize);
        let duplicates: Vec<char> = get_common_types(&first.to_string(), &second.to_string());
        let item_type = duplicates[0];

        Rucksack {item_type: item_type, items: raw}
    }
}

fn get_input() -> String {
    let path = "resources/day3input.txt";


    String::from_utf8(fs::read(path).unwrap()).unwrap()
}