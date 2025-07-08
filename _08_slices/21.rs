fn main() {
    let city = String::from("New Orleans");
    println!("{}", &city[5..9]); // prints "rlea"
    println!("{}", &city[5..=9]); // prints "rlea"

}

// N  e  w     O  r  l  e  a  n  s
// 0  1  2  3  4  5  6  7  8  9  10
