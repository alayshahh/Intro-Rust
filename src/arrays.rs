// Arrays - Fixed list where are the same data types

use std::mem;

pub fn run(){
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign values
    numbers[2] = 30;

    // Print whol array
    println!("{:?}", numbers);

    // get single val
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice 
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);



}