pub fn run() {
    // Prints Text to console. Easy
    println!("You can do this. Keep working hard and dont give up");

    // The curly brackets are filled in order left to right after the comma
    println!("{} is going to succeed in {}", "Brian", "Programming");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brian", "Germany", "Code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Brian",
        activity = "video games"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder debug trait
    println!("{:?}", (12, true, "Hello Broski"));

    // Basic Math
    println!("4 + 4 = {}", 4 + 4);
}
