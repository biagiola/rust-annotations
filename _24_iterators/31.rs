// Lecture: The enumerate method
// The enumerate adapter transforms an iterator so that if
// yields the index position along with the current element

fn main() {
    let applicants = vec!["Rob", "Bob", "Cob", "Alex", "Piers", "John", "Daniel"];

    let winners: Vec<&str> = applicants
        .into_iter()
        .enumerate()
        .filter(|(index, _)| index % 3 == 0)
        .map(|(_, applicant)| applicant)
        .collect();
    println!("{winners:?}");

    // notice we cannot use two times the winners
    // because .into_iter() takes ownership of winners
    let winners: Vec<&str> = applicants
        .into_iter()
        .enumerate()
        .filter_map(|(index, applicant)| {
            if index % 3 == 0 {
                Some(applicant)
            } else {
                None
            }
        })
        .collect();
    println!("{winners:?}");
}