// Lecture: Defining methods that accepts closure I
// Let's define our own method that will accept a closure as
// an argument. So we're going to define our closure type as a generic.
// Then we'll add trait bound and will require that that generic type
// implement one of the three FN traits.

use std::io::stdin;

#[derive(Debug)]
struct Vault {
    password: String,
    treasure: String,
}

impl Vault {
    fn unlock<F>(self, procedure: F) -> Option<String>
    where F: FnOnce() -> String,
    {
        let user_pass: String = procedure();

        if user_pass == self.password {
            Some(self.treasure)
        } else {
            None
        }
    }
}

fn main() {
    let vault = Vault {
        password: String::from("123456"),
        treasure: String::from("Gold"),
    };

    let mut user_input: String = String::new();
    println!("Please provide a password to crack the vault");
    stdin().read_line(&mut user_input);

    // there's no way hack being Fn or FnMut
    let hack = || user_input;
    let extraction: Option<String> = vault.unlock(hack);
    println!("{:?}", extraction);
}

// procedure closure can be only called once.