use std::env;

fn main() {
    println!("Tictactoe encoded in only 26 bits");

    let args: Vec<String> = env::args().collect();
    let input = &args[1].trim().parse::<u32>().unwrap();

    let first_part = (input >> 21) & 0b1111;
    let second_part = (input >> 17) & 0b1111;
    let third_part = (input >> 14) & 0b111;
    let fourth_part = (input >> 11) & 0b111;
    let fifth_part = (input >> 8) & 0b111;
    let sixth_part = (input >> 5) & 0b111;
    let seventh_part = (input >> 3) & 0b11;
    let eighth_part = (input >> 1) & 0b11;
    let ninth_part = input & 0b1;

    println!("{:04b}", first_part);
    println!("Second part (4 bits): {:04b}", second_part);
    println!("Third part (3 bits): {:03b}", third_part);
    println!("Fourth part (3 bits): {:03b}", fourth_part);
    println!("Fifth part (3 bits): {:03b}", fifth_part);
    println!("Sixth part (3 bits): {:03b}", sixth_part);
    println!("Seventh part (2 bits): {:02b}", seventh_part);
    println!("Eighth part (2 bits): {:02b}", eighth_part);
    println!("Ninth part (1 bit): {:01b}", ninth_part);
}
