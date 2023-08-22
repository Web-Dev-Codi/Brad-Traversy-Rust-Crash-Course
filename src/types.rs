// Primitive types --
// Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128(number of bits allocated to memory)
// Floats: f32, f64
// Boolean (bool)
// Characters (char)
// Tuples
// Arrays

// Rust is a statically typed language, which means it must know the types of all the variables
// at compile time. Sometimes Rust can infer the type bsed on what we used in the variables
// value.

pub fn run() {
    // Default is i32
    let x = 1;
    // Default f64
    let y = 2.5;
    // Explicit type
    let z: i64 = 3439548574;

    // MAX value of an Integer.
    let a = i64::MAX;

    println!("X: {} Y:{} Z:{} A:{}", x, y, z, a);
    println!("MAX i32: {}", std::i32::MAX);
    println!("MAX i64: {}", std::i64::MAX);

    // Boolean Explicitly setting
    let is_active: bool = true;

    // Getting a bool from an expression
    let is_greater = 10 > 5;

    // Characters must be in single quotes
    let a1 = 'a';
    // Emoji's
    let face = '\u{1F600}';

    // :? is debug mode
    println!("{:?}", (x, y, z, a, is_active, is_greater, a1, face))
}
