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
    let mut board = ['_'; 9];
    let mut turn = 'X'; // Start with 'X'

    fn place_move(board: &mut [char], index: usize, turn: char) -> bool {
        let mut skip_count = 1;
        for cell in board.iter_mut() {
            if *cell == '_' {
                if skip_count == index {
                    *cell = turn;
                    return true;
                }
                skip_count += 1;
            }
        }
        false
    }

    for (_i, &part) in parts.iter().enumerate() {
        let index = part as usize;
        if place_move(&mut board, index, turn) {
            turn = if turn == 'X' { 'O' } else { 'X' };
        }
    }

    for (i, &cell) in board.iter().enumerate() {
        if i % 3 == 0 {
            println!();
        }
        print!("{} ", cell);
    }
    println!();
}
