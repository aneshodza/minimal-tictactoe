use std::io;

mod decode;

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

            decode::draw_board(field);
        },
        "2" => {
            println!("Hasn't been implemented yet!");
        },
        "3" => {
            println!("Bye!");
        },
        &_ => {
            println!("Invalid input. Aborting");
        }
    }
}

