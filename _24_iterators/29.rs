// Lecture: Flatten method

fn main() {
    let spreadsheet = vec![
        [100, 200, 300],
        [234, 235, 236],
        [678, 679, 680],
    ];

    let values: Vec<i32> = spreadsheet.into_iter().flatten().collect();
    println!("{values:?}");
}