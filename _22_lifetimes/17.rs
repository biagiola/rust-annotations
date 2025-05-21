// Lecture: Multiple parameters II
// example error

fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn main() {
    let orlando = String::from("Orlando"); // we use deref coersion to parse to &str
    
    let result = {
        let san_francisco = String::from("San Francisco");
        longest(&orlando, &san_francisco) // <- borrowed value does not live long enough
        // here we have a possibillity of dangling reference coz, the we can return
        // san_francisco to set into the result variable, but we can delete that one
        // when we finish this scope. Result has the potencial of hold a dangling
        // reference.
        // To compile this code we have to make some adjustments in the next lecture.
    }
}

