impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let digits: Vec<_> = n.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
        
        let (mut sum, mut product) = (0, 1);
        for d in digits {
            sum += d;
            product *= d;
        }

        (product - sum) as i32
    }
}
