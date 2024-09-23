// https://leetcode.com/problems/missing-number/description/?envType=problem-list-v2&envId=hash-table

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        (1..=nums.len()).sum::<usize>() as i32 - nums.into_iter().sum::<i32>()
    }
}
