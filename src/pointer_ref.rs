// Reference Pointers - Point to a resource in memory
// use a variable to point to another variable.
pub fn run() {
    // With non_primitives, if you assign another variable to a piece of data, the first variable
    // will no longer  hold that value, You'll need to use a reference (&) to point to the resource.
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Vecs with & used as a reference: {:?}", (&vec1, vec2));
}
