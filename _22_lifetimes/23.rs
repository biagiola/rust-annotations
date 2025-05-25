// Lecture: Multiple lifetimes in structs

// We can use a single generic lifetime like 'a and mark both
// of our references with it, and this means that the lifetime
// of the travel plan struct must live within the overlapping
// lifetime of the reference of, from and to.
// But if the lifetimes are different, the borrow checker will
// pick the smaller one, or the overlapping one, where it can
// guarantee that both of the references are valid.

// let make the first case escenario example where we use just 'a
#[derive(Debug)]
struct TravelPlan<'a> {
    from: &'a str,
    to: &'a str
}

fn main() {
    let from = String::from("Portland");

    let plan: {
        let to = String::from("Bangor");

        let travel_plan: TravelPlan<'_> = TravelPlan {
            from: &from,
            to: &to, // The from variable has a longer lifetimes than the to variable but
            // the borow checker is going to pick the overlap or the smaller lifetime
            // of the two as the concrete lifetime that is filling in for generic lifetime 'a.
            // So that means the borrow checker is assuming that the travel plan struct can
            // only live as long as the 'to' referent lives and therefore the travel plan struct
            // will cease to be valid as well.
            // Technically there isn't a dangling reference but coz we have bound ourselves to a
            // single lifetime, the borrow checker has chosen the shorten lifetime to 'to' and
            // aired on the side of safety and said, I can't allow you to print the stuct out of
            // the to variable lifetime denoted by the 'a.
        };

        travel_plan.from
    };
    
    &str = String::from("Bangor")
    
    println!("{:#?}");
}