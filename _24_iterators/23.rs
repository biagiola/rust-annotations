// Lecture: The filter and find methods IV

fn main() {
    let numbers = [10, 13, 23, 2, 8, 9, 6];

    // let first_even: Option<i32> = numbers.into_iter().find(|number| number % 2 == 0);
    // let first_even: Option<&i32> = numbers.into_iter().find(|number| *number % 2 == 0);
    // let first_even: Option<i32> = numbers.into_iter().copied().find(|number| number % 2 == 0);
    
    // let first_even: Option<&i32> = numbers.iter().find(|number| *number % 2 == 0);
    let first_even: Option<i32> = numbers.iter().copied().find(|number| number % 2 == 0);
    println!("{first_even:?}");
    println!("{numbers:?}");

    let first_odd: Option<i32> = numbers.iter().copied().find(|number| number % 2 != 0);
    println!("{first_odd:?}");

    let nothing: Option<i32> = numbers.iter().copied().find(|number| *number > 100);
    println!("{nothing:?}");

    let last_even: Option<i32> = numbers.iter().copied().rfind(|number| number % 2 == 0);
    println!("{last_even:?}");

    let last_odd: Option<i32> = numbers.iter().copied().rfind(|number| number % 2 != 0);
    println!("{last_odd:?}");
}