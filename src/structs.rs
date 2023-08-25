struct Person {
    first_name: String,
    last_name: String,
}

// A implimentation of the Person Struct
impl Person {
    // Construct a person
    fn new(first: &str, last: &str) -> Person {
        Person {
            // Since we assigned String type we need to use to_string.
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&mut self) -> String {
        format!("{}{}", self.first_name, self.last_name)
    }

    // Set last Name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut p = Person::new("Gülden", "Bayar");
    println!("Last name: {}", p.full_name());
    p.set_last_name("Cordisco");
    println!(
        "Since I got married I changed my last name to {}",
        p.last_name
    );
    println!("Full Name with a tuple method(): {:?}", p.to_tuple());
}
