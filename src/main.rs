fn main() {
    print!("Tictactoe encoded in only 26 bits")

    // Take the input number and do right shifts + bitmasking to get the numbers
    // E.g: 0001|0100|010|001|100|100|11|01|1
    // 24 times rightfshift
    // Then 18 times rightshift gives 0001|0100 and bitmask with & 0000|1111 to get the number
    // Repeat that for all numbers
}
