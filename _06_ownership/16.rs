fn main() {
    let is_concert: bool = true;
    // bool implements Copy trait, so assignment creates a copy, not a move
    let mut is_event = is_concert;
    
    // Both variables are usable because bool was copied, not moved
    println!("{}", is_event);   
    println!("{}", is_concert); // still valid after assignment
    
    is_event = false;
    println!("{}", is_event);   
    println!("{}", is_concert); // unchanged
    
    // Case when we use heap variables
    // String does NOT implement Copy trait, so assignment moves ownership
    let concert_name: String = String::from("Rock Festival");
    let mut event_name = concert_name;
    
    println!("{}", event_name);     
    // println!("{}", concert_name); // ERROR: value borrowed here after move
    
    event_name = String::from("Jazz Night");
    println!("{}", event_name);     
    // println!("{}", concert_name); // ERROR: still can't use concert_name
}
