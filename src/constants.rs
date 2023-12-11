use std::collections::HashMap;

pub const BIT_SIZES: [u8; 9] = [4, 3, 3, 3, 3, 2, 2, 1, 0];

pub fn BIT_COUNTS() -> HashMap<u32, u8> {
    let mut array: HashMap<u32, u8> = HashMap::new();
    array.insert(5, 1);
    array.insert(9, 2);
    array.insert(12, 3);
    array.insert(16, 4);
    array.insert(19, 5);
    array.insert(21, 6);
    array.insert(23, 7);
    array.insert(25, 8);
    return array.clone();
}
