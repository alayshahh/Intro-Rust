// Vectors - Resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Re-assign values
    numbers[2] = 30;

    // Add to end of vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    // Print whol array
    println!("{:?}", numbers);

    // get single val
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Vector length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop over numbers in vector
    for x in numbers.iter() {
        println!("Number:{}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);

}
