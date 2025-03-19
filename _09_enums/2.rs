#[derive(Debug)]

enum PaymentMethodType {
    // Credit(String, i32, bool) // we can have multiple types
    CreditCard(String),
    DebitCard(String),
    Paypal(String)
}
fn main() {
    let visa: PaymentMethodType = {
        PaymentMethodType::CreditCard(
            String::from("0342-4392-603-0011")
        )
    };
    println!("{:?}", visa);
}