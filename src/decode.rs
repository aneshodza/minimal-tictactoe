use std::io;
use std::str::FromStr;

pub fn decode(input: u32) -> [u8; 9] {
    return [
        (input >> 21 & 0b1111) as u8,
        (input >> 17 & 0b1111) as u8,
        (input >> 14 & 0b111) as u8,
        (input >> 11 & 0b111) as u8,
        (input >> 8 & 0b111) as u8,
        (input >> 5 & 0b111) as u8,
        (input >> 3 & 0b11) as u8,
        (input >> 1 & 0b11) as u8,
        (input & 0b1) as u8,
    ];
}

pub fn decode_dialog() -> u32 {
    let mut field = String::new();

    println!("Please enter your game (in decimal)");

    loop {
        io::stdin()
            .read_line(&mut field)
            .expect("Failed to read line");

        match u32::from_str(field.trim()) {
            Ok(num) => {
                return num;
            },
            Err(_) => {
                println!("Please enter a valid number");
            }
        };
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_decoding() {
        let input = 2657435;

        let result = decode(input);

        assert_eq!(result, [1, 4, 2, 1, 4, 4, 3, 1, 1]);
    }
}
