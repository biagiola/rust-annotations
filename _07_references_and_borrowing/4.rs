fn main() {
    let mut coffe: String = String::from("Mocha");
    let a: &mut String = &mut coffe; // we're transfering ownership here
    // println!("{:p} {0}", &coffe); // so we can't print coffe anymore

    println!("{:p} {0}", &a);
    
    let b: &mut String = a;
    b.push_str(" and Cookies");
    println!("{:p} {b}", &b);

    println!("{a} {b}");
    // println!("{:p} {b}", &a); // and again, a transfered its owner to b

    // we can have just one mutable onwer at the same time
    // so we can't implement the copy trait. 
}