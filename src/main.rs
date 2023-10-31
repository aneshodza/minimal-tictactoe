use std::env;

fn main() {
    println!("Tictactoe encoded in only 25 bits");

    let args: Vec<String> = env::args().collect();
    let input = &args[1].trim().parse::<u32>().unwrap();

    // Extract the parts as described
    let parts = [
        (input >> 21) & 0b1111,
        (input >> 17) & 0b1111,
        (input >> 14) & 0b111,
        (input >> 11) & 0b111,
        (input >> 8) & 0b111,
        (input >> 5) & 0b111,
        (input >> 3) & 0b11,
        (input >> 1) & 0b11,
        input & 0b1,
    ];

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

