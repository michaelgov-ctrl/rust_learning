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

// https://leetcode.com/problems/first-letter-to-appear-twice/description/

use std::collections::HashSet;

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut acc: HashSet<char> = HashSet::new();
        s.chars().skip_while(|&c| acc.insert(c)).next().unwrap()
    }
}
    // with a for loop this could look like this
use std::collections::HashSet;

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut acc = HashSet::new();

        for c in s.chars() {
            if acc.contains(&c) {
                return c
            }
            
            acc.insert(c);
        }

        return 'f'
    }
}

// https://leetcode.com/problems/hash-divided-string/

impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        s.chars()
            .collect::<Vec<char>>()
            .chunks(k as usize)
            .map(|c| hash(c))
            .collect::<String>()
    }
}

fn hash(x: &[char]) -> char {
    (x.iter().fold(0, |acc, &c| (acc + c as u8 - 'a' as u8) % 26 ) + 'a' as u8) as char
}

// https://rust-lang.github.io/rfcs/2351-is-sorted.html
// is_sorted

fn main() {
    let v = vec![0];
    let is_sorted = (0..v.len()-1).all(|i| v[i] <= v[i+1]);
    println!("{is_sorted}")
}
