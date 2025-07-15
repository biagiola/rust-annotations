// This example shows that enum variants can store different kinds and structures
// of data, making them a very flexible way to group related types.

#[derive(Debug)]
enum PaymentMethodType {
    // "Tuple variants" hold anonymous data fields, like a tuple.
    CreditCard(String),
    DebitCard(String),
    // "Struct variants" hold named data fields, just like a struct.
    // This struct is defined inline as part of the `Paypal` variant itself.
    Paypal { username: String, password: String }
}

fn main() {
    let mut paypal_payment: PaymentMethodType = PaymentMethodType::CreditCard(
        String::from("9485-0594-9832-1200")
    );
    println!("{:?}", paypal_payment);
    // right now, our print will be: CreditCard("9485-0594-9832-1200")
    // in section 12 about enums option and result we see a lot on how to print the raw string

    paypal_payment = PaymentMethodType::Paypal {
        username: String::from("David Biagiola"),
        password: String::from("password123")
    };
    println!("{:#?}", paypal_payment);
}
