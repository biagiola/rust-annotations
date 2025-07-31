// Lecture: Multiple lifetimes in structs
// same example as before but now using other functions
// instead of inner scope blocks. Apply the same rules
// we're talked about.
// Remember that the end goal of the borrow checker is
// to make sure that we avoid dangling references and
// especially as you start passing in references in and
// out of functions.

#[derive(Debug)]
struct TravelPlan<'a, 'b> {
    from: &'a str,
    to: &'b str
}

fn figure_out_ending_point(from: & str) -> &str {
    let to = String::from("Bangor");

    let travel_plan: TravelPlan<'_, '_> = TravelPlan {
        from: &from,
        to: &to,
    };

    travel_plan.from
}

fn main() {
    let from = String::from("Portland");
    let plan: &str = figure_out_ending_point(&from);
    
    println!("{plan}");
}
