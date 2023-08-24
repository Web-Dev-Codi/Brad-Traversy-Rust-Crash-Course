// Vectors are resizable arrays.

use std::mem;

pub fn run() {
    // Define a vector using the Vec keyword and the type wit in angle brackets
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign a value
    numbers[2] = 40;

    // Add to the vector
    numbers.push(5);
    numbers.push(6);
    // TO print full Array
    println!("{:?}", numbers);

    // Get single value
    println!("Single Value: {}", numbers[2]);

    // Get Array length
    println!("vector length: {}", numbers.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //Get Slice from Array using referance
    let slice: &[i32] = &numbers[1..3];

    println!("Slices: {:?}", slice);

    //Loop through the vector
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Number Vec: {:?}", numbers);
}
