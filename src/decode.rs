use std::io;
use std::str::FromStr;

use crate::constants::{BIT_COUNTS, BIT_SIZES};

pub fn decode(input: u32) -> Vec<u8> {
    println!("{:0>25b}", input);
    let bit_count = input.ilog2() + 1;

    let mut move_count = *BIT_COUNTS().get(&bit_count).unwrap();
    if move_count == 8 {
        move_count += (input >> BIT_SIZES.iter().sum::<u8>() & 0b0001) as u8;
    }

    println!("{}", move_count);

    return Vec::new();
}

pub fn decode_dialog() -> u32 {
    let mut field = String::new();

    println!("Please enter your game (in binary)");

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
