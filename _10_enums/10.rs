// The match keyword IV
#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
                println!("Your item is being prepped for shipment")
            }
            OnlineOrderStatus::Delivered => {
                println!("Your item has been delivered")
            },
            other_status => {
                println!("Your item is {other_status:?}")
            } // this las case is the underscore one, the match default case.
        }
    }
}

fn main() {
    OnlineOrderStatus::Ordered.check();
    OnlineOrderStatus::Packed.check();
    OnlineOrderStatus::Delivered.check();
    OnlineOrderStatus::Shipped.check();
}

// This feature is part of Rust's comprehensive pattern matching system.
// The | acts as a logical OR within the pattern itself, allowing you to
// combine multiple possible values or patterns into a single arm of the match expression.
// Syntax: pattern_1 | pattern_2 | ... => expression
// Benefit: It eliminates the need for fall-through logic (like in C or C++)
// or repeating the same code block for multiple cases, making the code much cleaner and less error-prone.