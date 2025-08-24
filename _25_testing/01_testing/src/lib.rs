// >> cargo new --lib testing // this library crate has code that is intended to be used by other rust crates
// in this way we're going to have a lib.rs instead of a main.rs file.

// Our library it is called 'testing', you can see it on the Cargo.toml file

// To run the test we can execute one of these two commands:
// >> cargo test
// >> cargo t

// A unit test targets a small component of a program in isolation.
// A integration test tests the interation of multiple components within the program.

#[derive(Debug)]
struct Museum {
    paintings: Vec<String>,
    revenue: u32,
}

impl Museum {
    fn new() -> Self {
        Self {
            paintings: vec![],
            revenue: 0
        }
    }

    fn buy_painting(&mut self, painting: &str) {
        self.paintings.push(painting.to_string());
    }

    fn sell_ticket(&mut self) {
        self.revenue += 25;
    }

    fn has_impressive_collection(&self) -> bool {
        self.paintings.len() > 2
    }
}

// Is quite normal in the rust community to have the test of your component right into the same file.
// Right now our test section is a child module or nested module, so, in order to use "external" resources
// like the Museam struct with its implementation we need to imported into the test submodule.
// Also we're going to add the cfg (configuration) attribute that allows to avoid to include it when we're
// going to build the binary, and we pass the name of our test module that is 'test'.
#[cfg(test)]
mod test {
    // we are going to use some fns from the outside we maybe is better
    // to import then all and use later in a shorthand version.
    use super::*;
    // and also importing all sometimes is not a good practice due the name conflicts but
    // in this case is okay for our small library

    #[test]
    fn museum_sells_ticket_to_increase_revenue() {
        // whith the super keyword, we go one level up and
        // let mut museum = super::Museum::new();

        // and coz we're already imported above we dont use it again.
        let mut museum = Museum::new();

        // with the crate keyword, we are using the top root path from
        // our 'testing' library but because we only have one level down
        // is exacly the same as use super keyword
        // let mut museum = crate::Museum::new();
        museum.sell_ticket();

        // the order of the arguments does not matter in rust
        assert_eq!(museum.revenue, 25);

        // assert macro validates that some condition or vlaue is true
        assert_eq!(museum.has_impressive_collection(), false);
    }

    #[test]
    fn museum_can_have_impressive_art_collection() {
        let mut museum = Museum::new();
        museum.buy_paiting("Mona Lisa");
        museum.buy_paiting("The Starry Night");
        museum.buy_paiting("Girl with a Pearl Earring");
        museum.buy_paiting("The Starry Night");

        assert!(museum.has_impressive_collection());
    }
}
