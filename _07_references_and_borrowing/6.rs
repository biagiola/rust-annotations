// A dangling reference is a pointer to a memory address
// that has been deallocated
fn main() {
    
}
 
// here &city does not longer exists coz its scope has finished.
// Even before compile the file we can grab that error.
// we trying to return a ref that has been deallocated
fn create_city() -> &String {
    let city = String::from("New York");
    &city
}