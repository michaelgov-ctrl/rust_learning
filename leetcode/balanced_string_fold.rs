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


// https://leetcode.com/problems/find-closest-number-to-zero/description/

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((i32::MIN, i32::MAX), |(res, min), &x| {
                if x.abs() < min {
                    (x, x.abs())
                } else if x.abs() == min {
                    (res.max(x), min)
                } else {
                    (res, min)
                }
            })
            .0
    }
}
