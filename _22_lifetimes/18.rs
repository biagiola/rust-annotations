// Lecture: Multiple parameters II
// solution for the previous lesson

// In this way we're saying we're considering the lifetime of first param.
// In this case we don't need the 'b but we use it just for now.
// Now we're saying that both parameters has different independent lifetimes.
//
fn longest<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    // So coz we're return just first we cannot longer use this code,
    // remember our goal here now, is to make our main function compile
    // and understand about multiple lifetimes.
    // if first.len() > second.len() {
    //     first
    // } else {
    //     second
    // }

    println!("{second}"); // this is just to show we can use the param of course
    first // but now we telling with 'a we should return first

    // we're saying to the borrow checker now that the 'b doesn't matter for the 
    // sake of persisence. All the rust needs to guarantee is that the referent,
    // result, that the param 'first' is coming from, is where the return reference
    // is going to live within. So the borrow checker understands 'results' will
    // be a reference to effectively 'orlando' and not 'san_francisco'. That's all
    // said it in the function longest signature and the borrow checker knows it.

    // So san_francisco is going to go out of its scope, its lifetimes ends but 
    // there's no way we can have a dangling refernce to that thanks to our 
    // correct generic lifetimes annotations.
}

fn main() {
    let orlando = String::from("Orlando");
    
    let result = {
        let san_francisco = String::from("San Francisco");
        longest(&orlando, &san_francisco)
    }

}

