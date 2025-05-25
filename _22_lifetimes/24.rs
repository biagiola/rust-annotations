// Lecture: Multiple lifetimes in structs

// So the key difference here is that rust is able to figure
// out that the 'from' lifetime and the 'to' lifetime are 
// different, and it doesn't have to pick one to play the role
// of the single lifetime that we saw in the previous example.
// Rust understand now, that the 'from' lifetime will be the concrete
// lifetime for generic lifetime a, and the 'to' lifetime will be the
// concrete lifetime for generic b. And since it understands they're
// different and its associations of the fields, it know that if we
// return 'from', it know that from is the connected to a lifetime,
// thus it doesn't represent a dangling reference.
#[derive(Debug)]
struct TravelPlan<'a, 'b> {
    from: &'a str,
    to: &'b str
}

fn main() {
    let from = String::from("Portland");

    let plan: &str = {
        let to = String::from("Bangor");

        let travel_plan: TravelPlan<'_, '_> = TravelPlan {
            from: &from,
            to: &to,
        };

        travel_plan.from
    };
    
    println!("{plan}");
}