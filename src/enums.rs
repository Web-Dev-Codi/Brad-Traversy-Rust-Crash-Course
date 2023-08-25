// Enums are types which have a few definite values. Example: Ok or Error.

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Use the enum as a parameter
    // Action to perform
    // using match. similer to a switch statment in JS.
    match m {
        Movement::Up => println!("Avatar moved up"),
        Movement::Down => println!("Avatar moved down"),
        Movement::Left => println!("Avatar moved left"),
        Movement::Right => println!("Avatar moved right"),
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Right;
    let avatar3 = Movement::Down;
    let avatar4 = Movement::Up;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
