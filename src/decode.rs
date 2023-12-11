use std::io;
use std::str::FromStr;

use crate::constants::{BIT_COUNTS, BIT_SIZES};

pub fn decode(input: u32) -> Vec<u8> {
    let bit_count = input.ilog2() + 1;

    let mut move_count = *BIT_COUNTS().get(&bit_count).unwrap();
    if move_count == 8 {
        move_count += (input >> BIT_SIZES.iter().sum::<u8>() & 0b0001) as u8;
    }

    let mut shift_amout = 0;
    let mut result: Vec<u8> = Vec::new();

    while move_count > 0 {
        let bit_size = BIT_SIZES[(move_count - 1) as usize];
        let mask = (1 << bit_size) - 1;
        let new_input = (input >> shift_amout) & mask;
        result.insert(0, new_input as u8 + 1);
        shift_amout += bit_size;
        move_count -= 1;
    }

    return result;
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
        let input = 19378961;

        let result = decode(input);

        assert_eq!(result, [4, 7, 7, 4, 1, 3, 1, 2, 1]);
    }

    #[test]
    fn test_running_length_decoding() {
        let input = 3213;

        let result = decode(input);

        assert_eq!(result, [3, 2, 6]);
    }
}
