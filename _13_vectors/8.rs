fn main() {    
    // Vec works with all types and that include generics
    
    // 1. Basic types (we already saw these)
    let numbers: Vec<i32> = vec![1, 2, 3];
    let strings: Vec<String> = vec!["Hello".to_string(), "World".to_string()];
    let bools: Vec<bool> = vec![true, false, true];
    
    // 2. Complex types
    let tuples: Vec<(i32, &str)> = vec![(1, "one"), (2, "two"), (3, "three")];
    let nested_vecs: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4, 5]];
    
    // 3. Custom structs
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    
    let people: Vec<Person> = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
    ];
    
    // 4. Enums
    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue,
    }
    
    let colors: Vec<Color> = vec![Color::Red, Color::Green, Color::Blue];
    
    // 5. Options and Results (generics!)
    let values: Vec<Option<i32>> = vec![Some(1), None, Some(3)]; // here is hardcoded, but it can comming from a database for example
    let results: Vec<Result<i32, &str>> = vec![Ok(42), Err("failed"), Ok(100)];
    // . Vec<T> is a generic type, and we're using it with T = Option<i32> and T = Result<i32, &str>
    // . Option<T> is a generic type, and we're using it with T = i32
    // . Result<T, E> is a generic type, and we're using it with T = i32 and E = &str
    
    // 6. Generic functions with Vec
    // T can be any type that implements Debug.
    fn print_vec<T: std::fmt::Debug>(vec: Vec<T>) {
        println!("Generic vector: {:?}", vec);
    }
    
    print_vec(numbers); // Vec<i32>
    print_vec(people);  // Vec<Person>            -> Person is a struct
    print_vec(colors);  // Vec<Color>             -> Color is an enum
    print_vec(values);  // Vec<Option<i32>>       -> Option is a generic type
    print_vec(results); // Vec<Result<i32, &str>> -> Result is a generic type
    
    // 7. Vec can hold ANY type that implements required traits
    let mixed_data: Vec<Box<dyn std::fmt::Debug>> = vec![
        Box::new(42),
        Box::new("hello"),
        Box::new(true),
    ];
    // Box is a smart pointer that allows us to store data on the heap. It's a concept related to memory management
    // we are not disccussing in this entire course.
    // The dyn topic we'll disccuss again in the trait section.
    
    println!("Mixed data: {:?}", mixed_data);
    
    println!("\n🎯 ANSWER: Vec<T> is a GENERIC TYPE!");
    println!("- T can be ANY type: i32, String, structs, enums, etc.");
    println!("- Vec itself is generic: Vec<T> where T is the type parameter");
    println!("- You can have Vec<Vec<T>>, Vec<Option<T>>, Vec<Result<T, E>>, etc.");
    println!("- Vec works with your custom generics too!");
}
