// https://leetcode.com/problems/check-balanced-string/description/

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        num.as_bytes().into_iter().enumerate().fold(0, |sum, (i, e)| {
            if i % 2 == 0 {
                sum + e - b'0'
            } else {
                sum - e + b'0'
            }
        }) == 0
    }
}
