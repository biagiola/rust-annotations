// Generics in Enums
#[derive(Debug)]
enum Cheesesteak<T> {
    Plain,
    Topping(T)
}

fn main() {
    let mushroom: Cheesesteak<&str> = Cheesesteak::Topping("mushroom");
    println!("{:?}", mushroom);

    let onions: Cheesesteak<String> = Cheesesteak::Topping("onions".to_string());
    println!("{:?}", onions);
    
    let addon: String = "bacon".to_string();
    let bacon: Cheesesteak<&String> = Cheesesteak::Topping(&addon);
    println!("{}", addon);
    println!("{:?}", bacon);

    let mut plain: Cheesesteak<String> = Cheesesteak::Plain;
    plain = Cheesesteak::Topping(String::from("sausage"));
    plain = Cheesesteak::Topping(String::from("sausage".to_string()));
}