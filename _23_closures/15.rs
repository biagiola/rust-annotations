// Lecture: Fn traits II

fn execute_thrice<F>(procedure: F)
where
    F: Fn(), // Only allows closures that capture by *shared reference* and don't mutate
{
    procedure();
    procedure();
    procedure();
}

fn main() {
    let mut bosses = vec!["Boris"]; // `bosses` is mutable

    // This closure mutably borrows `bosses` and modifies it.
    let closure = || {
        bosses.push("Alexandra"); // mutation occurs
    };

    execute_thrice(closure); // will not compile
}
