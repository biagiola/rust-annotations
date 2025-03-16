fn main() {
    let ice_cream: &str = "Cookies and Cream";
    let dessert: &str = ice_cream;
    
    // we're having two reference in our example, the reference of 'dessert'
    // is a full copy of the reference from 'ice_cream', so now
    // both references the same "Cookies and Cream" text from the actual
    // binary.

    println!("{:p}", ice_cream);
    println!("{:p}", dessert);
}