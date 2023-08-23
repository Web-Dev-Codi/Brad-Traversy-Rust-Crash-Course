// Tuples group together values of a different types.
// Max 12 elements

pub fn run() {
    // Types must be predefined
    let person: (&str, &str, i8) = ("Gülden", "Berlin, Germany", 34);

    println!(
        "{0} is from {1} and is {2} years old.",
        person.0, person.1, person.2
    );
}
