fn main() {
    let mut animals = vec!["Giraffe", "Monkey", "Zebra"];
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals))
}

fn length_of_last_element(input: &mut Vec<&str>) -> Option<usize> {
    let last_element: &str = input.pop()?;
    Some(last_element.len())
}

// Side notes
// 1. let last_element: Option<&str> = input.pop();
// 2. Option::Some(last_element.len())