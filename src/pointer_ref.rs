// Reference Pointers - Point to a resource in memory
// use a variable to point to another variable.
pub fn run() {
    // Prim Array
    let array1 = [1, 2, 3, 4, 5, 6];
    let array2 = array1;

    println!("Values: {:?}", (array1, array2));
}
