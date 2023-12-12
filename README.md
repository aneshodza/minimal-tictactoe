# Minimal tictactoe
Have you ever wondered how small the needed storage for a game of tictactoe (including its history) is? This project tries to tackle that question
## Introduction
This project is a proof-of-concept on how small a game of tictactoe (including move history) can be encoded without any extra overhead like a Huffman-Tree.  
This project brings a game down to **25 bits**, but the storage is done in an u32 (unsigned 32bit integer), as machines cannot assign single bits.

## Trying the project
### 1. Getting the project
If you want to get the project onto your own machine, you have two options: Clone and build it yourself or download it for your machine.
#### Downloading the project
If you just want to download the project, please do that here for your machine:
* [Linux](https://drive.google.com/uc?export=download&id=1iX4Iq5KXK38x2_sv_qPkL7wAtGUq8nS8)
* [Windows (x86)](https://drive.google.com/uc?export=download&id=1W0BpVFOqLISQAX_Z3FP3guZf-doIEUuV)
* [MacOS (ARM)](https://drive.google.com/uc?export=download&id=10_o0qdmZh3KS1GVpC3deCeKLTo8-i92c)

**Keep in mind:** These were only tested for certain versions of the operating systems listed. There is a chance it
won't work for you.

#### Building the project
If you want to build it yourself, you first need to pull it:
```
git clone git@github.com:aneshodza/minimal-tictactoe.git
cd ../minimal-tictactoe
```
Then you build the project using `cargo`:
```
cargo build --release
cd target/release/
```

### 2. Running the project
To then run the project, you move into the folder you downloaded it into and run it using the following command:
```
./minimal-tictactoe
```
#### Permission issues
There is a possibility that it won't let you run the program, as its permissions are not correct. To fix that run:
```
chmod +x minimal-tictactoe
```
If you retry running the code, it should work.

### 3. Using the project
**Important:** Currently, the project doesn't support really playing the game, but rather just inputting the moves or game
that needs to be encoded or decoded.
After running the project, you will be greeted with the following screen:
```
Tictactoe encoded in only 25 bits
What do you want to do?
  1. Decode your game
  2. Encode a game
  3. Exit
```
Here you can choose between either encoding or decoding the game. For the instructions of each, please refer to the
following sections.
#### Encoding
When you want to encode a game, the following prompt will appear:
```
Tictactoe encoded in only 25 bits
What do you want to do?
  1. Decode your game
  2. Encode a game
  3. Exit
2
Please enter 1 to 9 numbers (1-9) separated by spaces:
```
There you need to give the program all the moves you made separated by spaces. The game from the explanation below
would look like this:
```
Please enter 1 to 9 numbers (1-9) separated by spaces:
1 4 5 3 2 3 2 1 1
```
Then the program will output the encoded game:
```
Your encoded number is: 2710131
It's binary representation: 0001010010101101001110011 (25 bit)

X O X
O O X
X O X
```
#### Decoding
When you want to decode a game, the following prompt will appear:
```
Tictactoe encoded in only 25 bits
What do you want to do?
  1. Decode your game
  2. Encode a game
  3. Exit
1
Please enter your game (in decimal)
```
There you need to give the program the encoded game. If we want to decode the game from the example above, we would
give it the following input:
```
Please enter your game (in decimal)
2710131
```
Then the program will output the decoded game:
```
X O X
O O X
X O X
```

## Encoding the game
### Getting the moves
The game numbers all squares on the ticktacktoe board from one to nine, as follows:
```
 1 | 2 | 3
-----------
 4 | 5 | 6
-----------
 7 | 8 | 9
```
When a user then plays a move, like on field 3, that index gets popped off the list, and the list gets
re-numbered:
```
 1 | 2 | X
-----------
 3 | 4 | 5
-----------
 6 | 7 | 8
```
And so-on and so forth for every of the 9 moves.

In the end, there should be an Array of nine numbers:
```
[1, 4, 5, 3, 2, 3, 2, 1, 1]
```
Every index of the array gets subtracted by one, as the binary representation can be shorter that way:
```
[0, 3, 4, 2, 1, 2, 1, 0, 0]
```
### Parsing them into a 25-bit number
#### Length of the moves
Every number occupies as little bits as are needed to represent all options.
As the board starts with nine options (zero to eight), the first number needs four bit.
The second then only needs three, as there are only eight options left (zero to seven), etc.
The constant `BIT_SIZES` is used to represent that:
```rust
pub const BIT_SIZES: [u8; 9] = [4, 3, 3, 3, 3, 2, 2, 1, 0];
```
#### Encoding the game length
## Drawbacks
The obvious drawback is that a Huffman-Tree would be much more efficient. Certain values are impossible or get skipped entirely. On top of that: This encoding is fixed-length, as a game can also end after only four moves. The games don't always go the entire way.  
The purpose of this project is not a practical encoding, but rather just a proof-of-concept on a thought experiment I had.

*Made with <3 by Anes Hodza*
