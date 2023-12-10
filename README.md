# Minimal tictactoe
Have you ever wondered how small the needed storage for a game of tictactoe (including its history) is? This project tries to tackle that question
## Introduction
This project is a proof-of-concept on how small a game of tictactoe (including move history) can be encoded without any extra overhead like a Huffman-Tree.  
This project brings a game down to **25 bits**, but the storage is done in a u32 (unsigned 32bit integer), as machines cannot assign single bits.

## How does it work?
The game numbers all fields on the tictactoe board from 1 to 9, as follows:
```
 1 | 2 | 3
-----------
 4 | 5 | 6
-----------
 7 | 8 | 9
```
When a user then plays a move, like on field 3, that index gets popped off the list and the list gets
re-numbered:
```
 1 | 2 | X
-----------
 3 | 4 | 5
-----------
 6 | 7 | 8
```
And so-on and so forth for every of the 9 moves.

In the end there should be an Array of 9 numbers:
```
[1, 4, 5, 3, 2, 3, 2, 1, 1]
```
And those numbers get parsed into a 25 bit number, where every number is put one after the other in binary:
```
0001 0100 101 011 010 011 10 01 1 = (2710131 in binary)
 1    4    5   3   2   3  2  1  1

The board:
  X O X
  O O X
  X O X
```
This works for both sides, as the number contains the game and its history.

## Drawbacks
The obvious drawback is that a Huffman-Tree would be much more efficient. Certain values are impossible or get skipped entirely. On top of that: This encoding is fixed-length, as a game can also end after only 4 moves. The games don't always go the entire way.  
The purpose of this project is not a pratical encoding, but rather just a proof-of-concept on a thought experiment I had.

*Made with <3 by Anes Hodza*
