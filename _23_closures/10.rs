// Lecture: Defining methods that accepts closure I
// variation of the previous example.

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

    // now our closure is Fn trait
    let hack = || {
        let mut user_input: String = String::new();
        println!("Please provide a password to crack the vault");
        stdin().read_line(&mut user_input);
        user_input.trim().to_string()
    };

    let extraction: Option<String> = vault.unlock(hack);
    println!("{:?}", extraction);
}

// side notes: now that the closure is Fn, it can be execute it
// multiple times. Each time it create a brand new string inside
// the closure. So there is no transfer of ownership or any capture
// of values from outside in our closure.
// So just because this says FnOnce does not mean that we are limited
// to passing in a FnOnce closure, we can pass that in, but also
// FnMut and Fn.

// Finanlly just a reminder on different ways on setting our types for
// the generics in our function signature
    // where clouse
    fn unlock<F>(self, procedure: F) -> Option<String>
    where F: FnOnce() -> String,

    // inline generics
    fn unlock<F: FnOnce() -> String>(self, procedure: F) -> Option<String>

    // using impl keyword
    fn unlock(self, procedure: F impl FnOnce() -> String) -> Option<String>