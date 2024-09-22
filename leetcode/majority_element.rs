// https://leetcode.com/problems/majority-element/description/?envType=problem-list-v2&envId=hash-table

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count_hashmap = std::collections::HashMap::new();

        let len = nums.len();
        let mut max = 0;
        for n in nums {
            let return_value = count_hashmap.entry(n).or_insert(0);
            *return_value += 1;

            if *return_value > max {
                max = *return_value;
                if max > len/2 {
                    return n
                }
            }
        }

        -1
    }
}
