// Arrays are a fixed size and need to be all the same data type.
use std::mem;

pub fn run() {
    // Must be staticly typed and the number of elements the array contains
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign a value
    numbers[2] = 40;
    // TO print full Array
    println!("{:?}", numbers);

    // Get single value
    println!("Single Value: {}", numbers[2]);

    // Get Array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //Get Slice from Array using referance
    let slice: &[i32] = &numbers;

    println!("Slices: {:?}", slice);
}
