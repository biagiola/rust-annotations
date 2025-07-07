fn main() {
    let mut s: &str = "Hello, Rust!";
    println!("{:p} {0}", s);

    s = "New String"; // ✅ Allowed: Changing the reference
    println!("{:p} {0}", s);

    // s.push_str(" World!"); // ❌ Error: `push_str` is not available for `&str` coz
    // it's immutable, just pointing to a new one.
}