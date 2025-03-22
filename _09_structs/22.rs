// Rust has three kind of structs: named fields, tuple-like and unit-like.
// We deal until now with the named field, now let's take a look on
// tuple-like also named. as tuple struct.
// It is a struct that assings each piece of data an order in line rather
// than a name
#[derive(Debug)]

// hours, minutes
struct ShortDuration(u32, u32);
// Years, months
struct LongDuration(u32, u32);

fn go_to_work(length: ShortDuration) {
    println!("Passing time {} hours and {} minutes", length.0, length.1)
}
// fn accept_tuple(u32, u32) {}

fn main() {
    let work_shift: ShortDuration = ShortDuration(8, 0);
    println!("{} Hours and {} minutes", work_shift.0, work_shift.1);

    let era: LongDuration = LongDuration(5, 3);
    println!("{} years and {} months", era.0, era.1);

    go_to_work(work_shift);
    // got_to_work(era); // we cannot pass era here, is another name
    // accept_tuple(era); // we cannot pass niether era or work_shift
}
