// https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut temp = n;
    let mut prod = 1;
    let mut sum = 0;
    while temp > 0 {
        let digit = temp % 10;
        sum += digit;
        prod *= digit;
        temp = temp / 10;
    }
    return prod - sum;
}