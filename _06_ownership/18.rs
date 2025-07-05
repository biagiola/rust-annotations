fn main() {
    let drink = String::from("Snapple");
    let mut beverage = drink;
    let mut delight = &beverage;

    // beverage is still the owner of "Snapple"
    // but delight is borrowing it
    // So, we cannot change beverage for now, coz
    // it is borrowed by delight currently.
    // beverage = String::from("Coca-Cola");

    // also, as we saw in the previous lecture,
    // we cannot make reference assignment to a borrowed value directly.
    // so, we need to create a new String and then assign it to delight, in that case.
    // delight = &String::from("Coca-Cola");
    println!("{}", delight);

    // now that delight is out of scope, we can change beverage
    beverage = String::from("Coca-Cola");
    println!("{}", beverage);
}