// Lecture: Fn traits III

// FnMut ensures it's safe and valid to call it multiple times
// while allowing mutation.
// Also procedure parameter change in each execution so we need
// to make it as mutable.
fn execute_thrice<F>(mut procedure: F)
where
    F: FnMut(),
{
    procedure();
    procedure();
    procedure();
}

fn main() {
    let mut bosses = vec!["Boris"];

    let closure = || {
        bosses.push("Alexandra");
    };

    execute_thrice(closure);
    println!("{bosses:?}")
}

// Like we said, Fn is very rare to see even in the rust standard library
// it's very strictive and FnMut allows to execute multiple times also, like
// the normal Fn without worry about Fn rules.