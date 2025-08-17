// Previously, we learned about adapter methods that create new iterators from
// existingones. Now, we’re focusing on consuming adapters—methods that exhaust
// an iterator to produce a new value. There are several ways to consume
// an iterator: using a for loop, manually calling the next method, or using
// methods like collect and fold. The methods discussed here, such as sum and reduce,
// also consume the iterator and return a single final value.

// All these consuming methods are available by default through the iterator trait.
// Once an iterator type defines the next method, Rust can automatically provide
// additional logic for calculations, like totals or products. For example, instead
// of manually implementing logic to sum values (as with reduce or fold), you can
// simply use the built-in sum method on an iterator of numeric values.
// This functionality is available out of the box, so you don’t need to write it yourself.

fn main() {
    let numbers = vec![4, 8, 15, 16, 32, 42];
    let total: i32 = numbers.iter().sum();
    println!("{total}");

    let product: i32 = numbers.iter().product();
    println!("{product}");

    // let max: Option<&i32> = numbers.iter().max();
    let max: &i32 = numbers.iter().max().unwrap();
    println!("{max}");

    let min: &i32 = numbers.iter().min().unwrap();
    println!("{min}");

    let count: usize = numbers.iter().count();
    println!("{count}");
    // let len: usize = count.len();
    // println!("{len}");

    // it's important to notice that the type matters
    // let invalid: f64 = 0.0 / 0.0;
    let numbers: Vec<f64> = vec![4.6, 8.8, 0.0 / 0.0, 6.2, f64::NAN]; // the last number is a NaN and how to you, for example order a NaN, but it at the end or at the beginnig?
    // numbers implement the order trait that indicates that a type can be ordered sequentially
    // and the order trait is a subtrait from PartialOrder trait, so, order trait is more stricter.
    // We cannot use methods like sum or max on an iterator of floating points where floating point
    // only implement partial order.
    println!("{numbers:?}");

    let total: f64 = numbers.iter().sum();
    println!("{total}");

    // it's better to filter all the NaN values first
    let total: f64 = numbers
        .iter()
        .filter(|number| !number.is_nan())
        .copied()
        .sum();
        // .fold(0.0, |total, current| total + current);
    println!("{total}");

    // 
    let max: Option<f64> = numbers
        .iter()
        // .filter(|numbers| !number.is_nan()) // the max automatically ignore NaN's
        .copied()
        .reduce(|accumulator| accumulator.max(current));
    println!("{max:?}");
}