// Lecture: Defining a method that accepts a closure II (FnMut trait)

#[derive(Debug)]
struct Location {
    name: String,
    treasures: u32,
}

struct Map<'a> {
    locations: &'a [Location],
}

impl<'a> Map<'a> {
    fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&Location),
    {
        let final_index: usize = self.locations.len() - 1;
        let mut current_index: usize = 0;

        while current_index <= final_index {
            let current_location = &self.locations[current_index];
            action(current_location); // this generate mutation on each iteration
            current_index += 1;
        }
    }
}

fn main() {
    let locations: [Location; 2] = [
        Location {
            name: String::from("Enchanted Forest"),
            treasures: 5
        },
        Location {
            name: String::from("Mystic Mountain"),
            treasures: 10
        },
    ];

    let map = Map { locations: &locations };
    let mut total_treasures = 0;

    map.explore(|location| {
        // rust will dereference the reference to
        // get to the location struct and get its
        // treasures field a u32, so we don't need to use the *
        total_treasures += location.treasures;
    });

    println!("Total treasures collected: {total_treasures}");

    // the beaty of the explorer method and the closure pattern
    // is that we can customize what happens on each invocation
    // of the explorer method.
    // For example we can still use the same method explore but
    // do total different operations using our closure
    let mut location_names: Vec<String> = Vec::new();

    map.explore(|location| {
        location_names.push(location.name.clone());
    });
    println!("Places to visit: {location_names:?}");

    // the only param the explore method receive is the actual closure body
    // that will be the action, the operation to be done in when we execute
    // it in the explore scope. The first time was to sum up integers from
    // struct fields and the second time was to push strings into a Vector
}

// Reminder on lifetimes: we need lifetime annotation,
// that's because our struct is storing a reference
// in its field, so this tells rust the lifetime of
// the maps will end before the lifetime of the 'Location' struct.
// We cannot have the 'Map' outlive the original data
// source, coz, if it does, then the locations field
// will be a dangling reference.

// Example of taking one value from the locations array.
// And if we pass just the whole locations arrays, which
// will still constitute a slice thanks to Deref coercion.
// let map = Map { locations: &locations[0..1] };
