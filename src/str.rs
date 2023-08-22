// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string
// data

pub fn run() {
    // Stored in stack
    let hello = "Hello";
    // String stored in the heap and is growable
    let yo = String::from("I am stored in the heap broski");

    println!(
        "This {} string literal is stored in the stack and {}",
        hello, yo
    );
}
