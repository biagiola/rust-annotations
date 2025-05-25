// Lecture: Lifetimes in structs
// example of a danling reference on structs

#[derive(Debug)]
struct TrainSystem<'a> {
    name: &'a str,
    // The struct must be deallocated first before the string slice is deallocated
    // to avoid dangling reference on the fields for the struct.
}

fn main() {
    
    let nj_transit: TrainSystem<'_> = {
        let name: String = String::from("NJ Transit");
        TrainSystem { name: &name }; // here the field of the struct
        // is being deallocated and the struct nj_trinsit outside must
        // to use it generating a dangling reference
    }

    println!("{nj_transit:#?}");
}