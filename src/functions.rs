// Functions - Used to store blocks of re-usable code.

pub fn run() {
    greeting("Hello", "Brian");

    // Bind functions values to varibles
    let bind = add(5, 9);
    println!("Binded Function to variable: {}", bind);

    //Closure
    let n3: i32 = 10;
    let add_num = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_num(8, 9));
}

// explicitly typed
fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
