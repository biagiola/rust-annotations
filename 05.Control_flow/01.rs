fn main() {
  let number: i32 = 8;

  match number {
    2 | 4 | 6 | 8 => println!("{number} is even"),
    1 | 3 | 5 | 7 => println!("{number} is odd"),
    _ => println!("Unknow number")
  }

  // short version
  match number {
    x: i32 if x % 2 == 0 => println!("{x} is even"),
    _ => println!("{y} is odd")
  }
}