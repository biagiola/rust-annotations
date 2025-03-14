fn main() {
	let mut name: String = String::from("Boris");
	name.push_str("Pask");
	// name variable lives in the places, the heap and the stack

	// At the beginnig
    // | OWNER     - VALUE |
    // | reference - x556b |
    // | length    - 5     | 
    // | capacity  - 10    |  | Boris    |
    // |------ STACK ------|  |-- HEAP --|
 
    // After the push
    // | OWNER     - VALUE |
    // | reference - x556b |
    // | length    - 10    | 
    // | capacity  - 10    |  | Boris Pask |
    // |------ STACK ------|  |--- HEAP ---| 

}