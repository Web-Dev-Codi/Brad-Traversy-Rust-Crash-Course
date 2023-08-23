// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string
// data

pub fn run() {
    // Stored in stack
    let mut hello = String::from("Hello ");
    //Get length
    println!("Legth: {}", hello.len());
    // Can only push on char
    hello.push('W');
    // Method to allocate more memory to push larger string stypes
    hello.push_str("orld!");
    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    println!("Is Empty: {}", hello.is_empty());

    println!("Contains: {}", hello.contains("World"));

    //Loop through String by whitespace
    for put_anything_here in hello.split_whitespace() {
        println!("{}", put_anything_here);
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);

    s.push('b');
    s.push('a');

    // Assertions....left equal to the right
    // Failing to show error
    assert_eq!(3, s.len());

    println!("{}", s);

    println!("{}", hello);
}
