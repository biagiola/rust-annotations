// Lecture: Multiple parameters II

// the 'a on the first and second parameter doesn't mean that
// they have the same lifetime. We are just saying that the 
// return value share the region of code, of lifetime of we
// the two parameters lives too. To avoid dangling reference
// returned reference must live within the shared region of code
// that both parameters represent. Similar like a Venn Diagram,
// where first and second are outer circles and where the lifetime
// 'a is the shared part in the middle.
fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn main() {
    let orlando = String::from("Orlando"); // we use deref coersion to parse to &str
    let san_francisco = String::from("San Francisco");
    let result = longest(&orlando, &san_francisco);
    println!("{result}");
}

// the lifetime of orlando is from line  20 to the end
// and liftime of san_francisco is from 21 to the end
// so the 'a will be the shortest, the san_francisco one,
// to make sure no one excees our outlives the lifetime of
// the other variable and thus guarantee tht a reference 
// to either one is going to a valid one.