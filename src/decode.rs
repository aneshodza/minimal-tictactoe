use std::io;
use std::str::FromStr;

use crate::constants::BIT_SIZES;

pub fn decode(input: u32) -> Vec<u8> {
    let bit_count: u8 = (input.ilog2() + 1) as u8;

    let mut move_count = find_index(bit_count) + 1;

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

fn find_index(bit_count: u8) -> u8 {
    let mut sum = 1;
    for (index, &value) in BIT_SIZES.iter().enumerate() {
        sum += value as u8;
        if sum == bit_count {
            return index as u8;
        }
    }
    return 0; // if bit_count is not reached
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
        let input = 5203490;

        let result = decode(input);

        assert_eq!(result, [4, 7, 7, 4, 1, 3, 1, 2, 1]);
    }

    #[test]
    fn test_running_length_decoding() {
        let input = 1165;

        let result = decode(input);

        assert_eq!(result, [3, 2, 6]);
    }

    #[test]
    fn test_find_index() {
        let input = 11;
        let result = find_index(input) + 1;
        assert_eq!(result, 3);
    }
}
