fn main() {
    let ice_cream: &str = "Cookies and Cream";
    let dessert: &str = ice_cream;
    
    // we have two references in our example, the reference of 'dessert'
    // is a full copy of the reference from 'ice_cream', so now
    // both reference the same "Cookies and Cream" text from the actual
    // binary.

    println!("{:p}", ice_cream);
    println!("{:p}", dessert);
    // Both will print the same address since they point to the same location
    // in the program's data segment
}