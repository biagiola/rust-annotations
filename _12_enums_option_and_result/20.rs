// Building a Option from the scratch called MyOption

// To emulate the buit-in Option of Rust, we need Copy for i32 but also Clone for String for example or to be clone when we pass through fn's arguments
#[derive(Debug, Copy, Clone)]
enum MyOption {
    Some(i32),
    None
}

impl MyOption {
    fn unwrap(self) -> i32 { // and the self here is implementing a copy trait thanks to the directives declared above so alfter invocation, user doesn't loose ownership.
        match self { // self in this case is a instance of the enum.
            MyOption::Some(value) => value,
            MyOption::None => panic!("Uh oh!!") // in the case of MyOption we cann't ommit its prefix. Just the built-in rust option could be that due the top level options variants, MyOption is a custom one.
        }
    }

    fn unwrap_or(self, fallback_value: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value
        }
    }
}

fn main() {
    let some_option2: MyOption = MyOption::Some(100);
    println!("{}", some_option2.unwrap_or(13)); // TODO: this fn will take ownership but over a copy so we're not gonna loose the original enum.

    let some_option2: MyOption = MyOption::None;
    println!("{}", some_option2.unwrap_or(13))
}