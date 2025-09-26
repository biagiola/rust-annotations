fn main() {
    let a = 1;
    {
        let b = 2;
        println!("a is valid here {}", a);
    }

    println!("b is invalid here");

    let c = String::from("Winter");
    let d = c;

    println!("d took ownership of c {}", d);
    println!("so c does not exists anymore");
}
