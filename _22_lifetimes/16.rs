// Lecture: Multiple parameters II

fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn main() {
    let orlando = String::from("Orlando"); // we use deref coersion to parse to &str
    
    {
        let san_francisco = String::from("San Francisco");
        let result = longest(&orlando, &san_francisco); // they still overlap thier lifetimes
        // this will be a valid reference regarless of which string it is providing a reference to.
        println!("{result}");

    }
    // 1000 lines of code making orlando lifetime way bigger than san_francisco's lifetime
    // so orlando and san_francisco doesn't have the same concrete lifetime.
    // They don't have to have the exact same lifetime due the 'a, that what 'a actually, 
    // also means, that the overlap life between them, the region for which they exists,
    // the return value must to live within that region or lifetime, because that's the
    // only way that we're going to guarantee the avoidance of dangling reference.
    println!("{orlando}");
}

