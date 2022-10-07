#![allow(dead_code)]

#[derive(Debug)]
struct Order {
    items: Vec<String>,
    quantities: Vec<i32>,
    prices: Vec<f32>,
    status: Status,
}

#[derive(Debug)]
enum Status {
    OPEN,
    PAID,
}

impl Order {
    fn new() -> Self {
        Self {
            items: vec![],
            quantities: vec![],
            prices: vec![],
            status: Status::OPEN,
        }
    }

    fn add_item(&mut self, item_name: String, quantity: i32, price: f32) {
        self.items.push(item_name);
        self.quantities.push(quantity);
        self.prices.push(price);

    }
}

trait PaymentProcessor {
    fn pay(&self, order: &mut Order, security_code: String);
}

struct DebitPaymentProcessor;

impl DebitPaymentProcessor {
    fn new() -> Self {
        return Self;
    }

}

impl PaymentProcessor for DebitPaymentProcessor {
    fn pay(&self, order: &mut Order, security_code: String) {
        println!("Processing debit payment");
        println!("Verifying security code: {:?}", security_code);
        order.status = Status::PAID;
    }
}


struct CreditPaymentProcessor;

impl CreditPaymentProcessor {
    fn new() -> Self {
        return Self;
    }
}

impl PaymentProcessor for CreditPaymentProcessor {
    fn pay(&self, order: &mut Order, security_code: String) {
        println!("Processing credit payment");
        println!("Verifying security code: {:?}", security_code);
        order.status = Status::PAID;
    }

}

fn main() {
    let mut order = Order::new();
    //println!("{:#?}", order);
    order.add_item("KeyBoard".to_owned(), 2, 3212.0);
    //println!("{:#?}", order);
    let pay_processor = DebitPaymentProcessor::new();
    pay_processor.pay(&mut order,"3fce4d23".to_string());
    println!("{:#?}", order);
}
