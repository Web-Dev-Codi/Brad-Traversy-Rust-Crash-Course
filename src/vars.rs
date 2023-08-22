// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block scoped language

pub fn run() {
    let name = "Gülden";
    let mut age = 34;
    println!("My beautiful ladies name is {} and she is {}", name, age);
    age = 35;
    println!("My beautiful ladies name is {} and she is {}", name, age);

    // defining a const
    // When defining a const you need to explicitly define a type.
    const ID: i32 = 001;

    println!("ID: {}", ID);

    // Assign multiple variable at once
    let (my_name, my_age) = ("Brian", 40);
    println!("{} is {} years old. What the F happend?", my_name, my_age);
}
