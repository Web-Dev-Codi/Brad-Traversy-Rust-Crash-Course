use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Gülden";
    let status = "100%";

    if command == "hello" {
        println!("Hi {}, How is your day going?", name);
    } else if command == "status" {
        println!("Status: {}", status);
    } else {
        println!("Not a valid command");
    }
}
