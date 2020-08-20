// https://leetcode.com/problems/apply-discount-every-n-orders/
pub trait Employee {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self;
    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64;
}

pub struct Cashier {
    threshold: i32,
    orders_processed: i32,
    discount: i32,
    products: Vec<i32>,
    prices: Vec<i32>,
}

impl Employee for Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        return Cashier {
            threshold: n,
            orders_processed: 1,
            discount,
            products,
            prices,
        };
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        if product.len() != amount.len() {
            panic!("Invalid args!");
        }
        let mut bill_amount: f64 = product
            .into_iter()
            .enumerate()
            .map(|(_, prod)| {
                return self.products
                    .clone()
                    .into_iter()
                    .position(|p| { p == prod })
                    .unwrap();
            })
            .enumerate()
            .fold(0.0, |mut acc, (index, prod)| {
                let price = self.prices[prod];
                let quantity = amount[index];
                acc += (quantity * price) as f64;
                return acc;
            });
        if self.orders_processed % self.threshold == 0 {
            bill_amount -= (self.discount as f64 * bill_amount) / 100.0;
        }
        self.orders_processed += 1;
        return bill_amount;
    }
}