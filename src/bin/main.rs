fn main() {
    let choice : u8;
    unsafe {
        let mut s = String::from("B X");
        let v: &mut Vec<u8> = s.as_mut_vec();
        choice = v[0] - ('A' as u8);
    }
    println!("{}", choice);
}
