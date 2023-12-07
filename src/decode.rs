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

pub fn draw_board(parts: [u8; 9]) {
    // Initialize the Tic-Tac-Toe board
    let mut board = [' '; 9];
    let mut turn = 'X'; // Start with 'X'

    // Function to place moves on the board
    fn place_move(board: &mut [char], index: usize, turn: char) -> bool {
        let mut skip_count = 0;
        for cell in board.iter_mut() {
            if *cell == ' ' {
                if skip_count == index {
                    *cell = turn;
                    return true;
                }
                skip_count += 1;
            }
        }
        false
    }

    // Iterate through the parts and place the moves
    for (i, &part) in parts.iter().enumerate() {
        // Convert part to 0-based index
        let index = part as usize - 1;

        // Try to place the move on the board
        if place_move(&mut board, index, turn) {
            // If successful, switch turns
            turn = if turn == 'X' { 'O' } else { 'X' };
        } else {
            println!("Could not place move for part {}: {}", i + 1, part);
        }
    }

    // Print the Tic-Tac-Toe board
    for (i, &cell) in board.iter().enumerate() {
        if i % 3 == 0 {
            println!(); // Newline for new row
        }
        print!("{} ", cell);
    }
    println!(); // Newline at the end of the board
}
