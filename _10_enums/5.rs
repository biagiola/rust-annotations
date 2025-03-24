// Struct variants
#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    Paypal { username: String, password: String }
    // here is where we use the struct vairant.
    // A main difference is that we have the struct
    // decouple, we have its type to use elsewhere
}

fn main() {
    let mut paypal_payment: PaymentMethodType = PaymentMethodType::CreditCard(
        String::from("9485-0594-9832-1200")
    );

    paypal_payment = PaymentMethodType::Paypal {
        username: String::from("David Biagiola"),
        password: String::from("password123")
    };
    println!("{:#?}", paypal_payment);
}