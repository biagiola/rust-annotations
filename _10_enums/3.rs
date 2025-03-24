#[derive(Debug)]

enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    Paypal(String, String)
}
fn main() {
    let mut payment_method: PaymentMethodType =
        PaymentMethodType::CreditCard(
            String::from("0342-4392-603-0011")
        );

    payment_method = PaymentMethodType::Paypal(
        String::from("david@gmail.com"),
        String::from("password")
    );
    println!("{:?}", payment_method);
}