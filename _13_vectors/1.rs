fn main() {
    // arrays are fixed data type saved into the stack
    let rick_moranis_movies: [&str; 3] = ["Ghostbusters", "Honey, I Shrunk the Kids", "Spaceballs"];

    // Vector in the other hand, are heap data type the Vec is in the rust prelude.
    // We have to provide the type argument: i32, String.
    let pizza_diameter: Vec<i32> = vec![8, 12, 14];
    println!("{:?}", pizza_diameter);

    let pastas = vec!["Rigatoni", "Angel hair", "Fettucine"];
    println!("{:?}", pastas);

    // if we know the sizes will not change, it's better to use arrays
}

// turbofish style
// let mut pizza_diameter = Vec::<i32>::new();
// pizza_diameter.push(8);
// pizza_diameter.push(12);
// pizza_diameter.push(14);

// let pastas: Vec<&str> = Vec::new("");
// let pastas = Vec::<&str>::new();