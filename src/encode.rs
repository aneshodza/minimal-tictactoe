use std::io;
use std::str::FromStr;

use crate::constants::BIT_SIZES;

pub fn encode(moves: &Vec<u8>) -> u32 {
    let mut output: u32 = 0;
    let length = moves.len();
    let mut shift: u8 = BIT_SIZES[..length].iter().sum();

    output |= (length as u32) << shift;
    shift -= BIT_SIZES[0];

    for (idx, element) in moves.iter().enumerate() {
        output |= (((element - 1) as u32) << shift) as u32;
        if shift > 0 {
            shift -= BIT_SIZES[idx + 1];
        }
    }

    println!("{}", output);
    println!("{:0>25b}", output);
    let bit_count = output.ilog2() + 1;

    println!("bitcount: {}", bit_count);
    return output;
}

pub fn encode_dialog() -> Vec<u8> {
    println!("Please enter 1 to 9 numbers (1-9) separated by spaces:");

    loop {
        let mut input_line = String::new();

        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        let parts: Vec<u8> = input_line
            .trim()
            .split_whitespace()
            .filter_map(|s| u8::from_str(s).ok())
            .collect();

        if parts.len() <= 9 && parts.len() >= 1 {
            return parts;
        } else {
            println!("Invalid input. Please enter exactly 9 numbers separated by spaces.");
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_encoding() {
        let input = [1, 4, 2, 1, 4, 4, 3, 1, 1];

        let result = encode(&input.to_vec());

        assert_eq!(result, 18925692);
    }
}
