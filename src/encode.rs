use std::io;
use std::str::FromStr;

pub fn encode(moves: [u8; 9]) -> u32 {
    let mut output: u32 = 0;

    output |= (moves[0] as u32) << 21;
    output |= (moves[1] as u32) << 17;
    output |= (moves[2] as u32) << 14;
    output |= (moves[3] as u32) << 11;
    output |= (moves[4] as u32) << 8;
    output |= (moves[5] as u32) << 5;
    output |= (moves[6] as u32) << 3;
    output |= (moves[7] as u32) << 1;
    output |= moves[8] as u32;

    return output;
}

pub fn encode_dialog() -> [u8; 9] {
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
            let mut array = [0u8; 9];
            let num_elements_to_copy = parts.len();
            array[..num_elements_to_copy]
                .copy_from_slice(&parts[..num_elements_to_copy]);
            return array;
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

        let result = encode(input);

        assert_eq!(result, 2657435);
    }
}
