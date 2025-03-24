// Struct variants
// A struct variant stores associated data in field rather than by position.
// Each piece of data has an associated name.
#[derive(Debug)]
struct Credential {
    username: String,
    password: String
}

#[derive(Debug)]
struct PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    Paypal(Credentials)
}

fn main() {
    let credentials: Credentials = Credentials {
        username: String::from("David Biagiola"),
        password: String::from("password123")
    }
    let mut paypal: PaymentMethodType = PaymentMethodType::Paypal(
        credentials
    );
    println!("{:#?}", paypal);
}