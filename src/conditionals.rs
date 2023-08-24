// Checks if conditions are true or false and acts on it
pub fn run() {
    let age: u8 = 30;
    let check_id: bool = true;
    let knows_person_of_age: bool = true;

    //If/else Statment
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?")
    } else if age < 21 && check_id {
        println!("Bartender: Sorry come back when you are of age.")
    } else {
        println!("Bartender: I'll need to see your ID.")
    }

    // Shorthand if statement
}
