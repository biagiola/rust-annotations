// cargo new --lib testing // this library crate has code that is intended to be used by other rust crates
// in this way we're going to have a lib.rs instead of a main.rs file.

// A unit test targets a small component of a program in isolation.
// A integration test tests the interation of multiple components within the program.

#[derive(Debug)]
struct Museam {
    paintings: Vec<String>,
    revenue: u32,
}

impl Museam {
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
        self.paintings.len() > 2;
    }
}

// Is quite normal in the rust community to have the test of your component right into the same file.
// Right now our test section is a child module or nested module, so, in order to use "external" resources
// like the Museam struct with its implementation we need to imported into the test submodule
mod test {
    #[test]
    fn museum_sells_ticket_to_increase_revenue() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 25);
    }
}
