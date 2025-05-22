// Lecture: Multiple parameters part II
// Solution for the previous lesson

// In this way we're saying we're considering the lifetime of the first param.
// In this case, we don't need the 'b, but we use it just for demonstration purposes.
// Now we're saying that both parameters have different, independent lifetimes.
//
fn longest<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    // So, because we're returning just `first`, we can no longer use this code
    // exactly how we used it in the previous example.
    // if first.len() > second.len() {
    //     first
    // } else {
    //     second
    // }
    // Remember, our goal now is to make our main function compile and understand multiple lifetimes.
    
    println!("{second}"); // Lifetime doesn't affect the params at runtime, so we can keep using it,
    first // but now we're telling with 'a that we should return `first`

    // We're saying to the borrow checker now that 'b doesn't matter for the
    // sake of persistence. All Rust needs to guarantee is that the referent—
    // the `result` variable, where the param `first` is coming from—is where the return
    // reference (`first`) is going to live within.
    // So the borrow checker understands that `result` will be a reference to, effectively,
    // `orlando` and not `san_francisco`. That's all said in the function `longest`'s
    // signature, and the borrow checker knows it.

    // So, `san_francisco` is going to go out of its scope—its lifetime ends—but
    // there's no way we can have a dangling reference to that, thanks to our
    // correct generic lifetime annotations.
}

fn main() {
    let orlando = String::from("Orlando");
    
    let result: &str = {
        let san_francisco = String::from("San Francisco");
        longest(&orlando, &san_francisco)
    };
}
