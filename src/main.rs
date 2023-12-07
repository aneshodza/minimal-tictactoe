use std::io;

mod decode;
mod encode;

fn main() {
    println!("Tictactoe encoded in only 25 bits");
    println!("What do you want to do?");
    println!("  1. Decode your game");
    println!("  2. Encode a game");
    println!("  3. Exit");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => {
            let input = decode::decode_dialog();
            let field = decode::decode(input);

            draw_board(field);
        },
        "2" => {
            let moves = encode::encode_dialog();
            let encoded_number = encode::encode(moves);

            println!("Your encoded number is: {}", encoded_number);
            draw_board(moves);
        },
        "3" => {
            println!("Bye!");
        },
        &_ => {
            println!("Invalid input. Aborting");
        }
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
