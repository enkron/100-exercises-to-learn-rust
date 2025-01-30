// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

#[allow(dead_code)]
impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        Self::product_name_validation(&product_name);
        Self::zero_validation(quantity, unit_price);

        Self {
            product_name,
            quantity,
            unit_price,
        }
    }

    fn product_name_validation(p: &String) {
        if p.is_empty() || p.len() > 300 {
            panic!();
        }
    }

    fn zero_validation(q: u32, u: u32) {
        if q == 0 || u == 0 {
            panic!()
        }
    }

    pub fn product_name(&self) -> &String {
        Self::product_name_validation(&self.product_name);

        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        Self::zero_validation(self.quantity, self.unit_price);

        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn set_quantity(&mut self, q: u32) {
        self.quantity = q;
    }

    pub fn set_product_name(&mut self, n: String) {
        self.product_name = n;
    }

    pub fn set_unit_price(&mut self, u: u32) {
        self.unit_price = u;
    }
}
