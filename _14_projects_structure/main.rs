// A package is a collection of one or more crates.
// A crate is a collection of Rust code that produces an executable or a library.
// A crate is the smallest amount of code that the Rust compiler considers at a time.

// There are two categories of crate
// 1. A binary crate: is a crate that compiles to an executable. It has a main function
//    that is the entrypoint for the executable
// 2. A library crate: exports funcionality for other Rust programs to share and use.
//    It does not have a main function and does not compile to be an executable program

// In summary, a rust project is a folder with a Cargo.toml file, also called as package,
// and that package must to have at least one create, which can be either a binary crate
// or a library crate, at least one of them.
mod inventory;
mod orders;

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        inventory::MANAGER,
        orders::MANAGER,
        inventory::FLOOR_SPACE
    );

    inventory::talk_to_manager();

    let favorite_category = inventory::products::ProductCategory::Hammer;
    println!("My fav category of item is {favorite_category:?}");

    let tall_lader = inventory::products::Item {
        name: String::from("Ladder-o-matic 2000"),
        category: favorite_category,
        quantity: 100
    };
    println!("{:#?}", tall_lader);
}