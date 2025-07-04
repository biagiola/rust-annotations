fn main() {
	let mut name: String = String::from("Boris");
	name.push_str(" Pask");
	// name variable lives in two places, the heap and the stack
    // the string metadata is stored in the stack and the value is stored in the heap

	// At the beginnig
    // | OWNER     - VALUE |
    // | reference - x556b |
    // | length    - 5     | 
    // | capacity  - 5     |  | Boris    |
    // |------ STACK ------|  |-- HEAP --|
 
    // After the push
    // | OWNER     - VALUE |
    // | reference - x556b |
    // | length    - 10    | 
    // | capacity  - 10    |  | Boris Pask |
    // |------ STACK ------|  |--- HEAP ---| 

}
