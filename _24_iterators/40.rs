// Lecture: is_sorted and sort method

fn main() {
    let mut points = [3, 8, 1, 11, 5];
    println!("{}", points.is_sorted());

    points.sort();

    println!("{points:?}");
    println!("{}", points.is_sorted());

    points.reverse();
    
    println!("{points:?}");
    println!("{}", points.is_sorted());

    let mut exercises = ["squat", "Bench", "Deadlift"];
    exercises.sort();
    println!("{exercises:?}");

}