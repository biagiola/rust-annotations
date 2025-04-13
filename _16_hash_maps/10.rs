use std::collections::HashSet;
 
fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();
    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Boris");
    concert_queue.insert("Melissa");
    
    movie_queue.insert("Boris");
    movie_queue.insert("Phil");
    
    println!("{:?}", concert_queue.union(&movie_queue)); // return a Union struct
    println!("{:?}", movie_queue.union(&concert_queue)); // exact same elements
    
    println!("{:?}", concert_queue.difference(&movie_queue)); // Melissa
    println!("{:?}", movie_queue.difference(&concert_queue)); // Phil
    
    // either one of the sets but not boths
    println!("{:?}", concert_queue.symmetric_difference(&movie_queue)); // ["Mellisa", "Phil"]
    println!("{:?}", movie_queue.symmetric_difference(&concert_queue)); // ["Phil", "Melissa"]   

    // return a true if they don't have any values in common. Does not matter the other
    println!("{:?}", concert_queue.is_disjoint(&movie_queue)); // ["Mellisa", "Phil"]
    println!("{:?}", movie_queue.is_disjoint(&concert_queue)); // ["Phil", "Melissa"]
    
    let mut attendees = HashSet::new();
    attendees.insert("Boris");
    println!("{:?}", attendees.is_subset(&movie_queue));
    println!("{:?}", movie_queue.is_superset(&attendees));
}