
#[derive(Debug)]
pub struct Order {
    pub items: Vec<String>,
    pub quantities: Vec<i32>,
    pub prices: Vec<f32>,
    pub status: Status,
}

#[derive(Debug)]
pub enum Status {
    OPEN,
    PAID,
}

impl Order {
    pub fn new() -> Self {
        Self {
            items: vec![],
            quantities: vec![],
            prices: vec![],
            status: Status::OPEN,
        }
    }

    pub fn add_item(&mut self, item_name: String, quantity: i32, price: f32) {
        self.items.push(item_name);
        self.quantities.push(quantity);
        self.prices.push(price);

    }
}
