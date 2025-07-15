// The match Keyword V - Using Option<T> for optional variant data.
// We can use Rust's built-in `Option` enum to represent a value that
// might be present or absent (like null/empty).
enum Milk {
    // By wrapping the `i32` in an `Option`, `Lowfat` can now represent
    // milk with a specific percentage, or with an unknown percentage.
    Lowfat(Option<i32>),
    Whole,
}

impl Milk {
    // just a side note, we're going to transfer ownership
    // but in this case doesn't care us.
    fn drink(self) {
        match self {
            // Match on a specific value *inside* the `Some`.
            Milk::Lowfat(Some(2)) => {
                println!("Delicious, 2% milk is my favorite!");
            }
            // Match on any other value inside a `Some`.
            Milk::Lowfat(Some(percent)) => {
                println!("You've got lowfat {percent}% milk.");
            }
            // Match on the `None` case, where the value is absent.
            Milk::Lowfat(None) => {
                println!("You've got some lowfat milk of an unknown percentage.");
            }
            Milk::Whole => {
                println!("You've got whole milk!");
            }
        }
    }
}

fn main() {
    Milk::Lowfat(Some(1)).drink();
    Milk::Lowfat(Some(2)).drink();
    // To represent an "empty" or "null" percentage, we now use `None`.
    Milk::Lowfat(None).drink();
    Milk::Whole.drink();
}

// we will have more examples on match and options for null variants we are going to discuss in section 12.