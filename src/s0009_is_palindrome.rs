/// https://leetcode.com/problems/palindrome-number/
///
/// Given an integer x, return true if x is a palindrome, and false otherwise.
//
//
// Example 1:
//
// Input: x = 121
// Output: true
// Explanation: 121 reads as 121 from left to right and from right to left.
//
//
// Example 2:
//
// Input: x = -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
//
//
// Example 3:
//
// Input: x = 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

struct Solution;

// =============================================================

impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        } else {
            let mut a = x;
            let mut b = 0;

            while a > 0 {
                b = b * 10 + a % 10;
                a /= 10;
            }

            if x == b {
                return true;
            }
        }
        return false;
    }
}

// =============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, 1 + 1);
        assert_eq!(true, Solution::is_palindrome(121));
        assert_eq!(false, Solution::is_palindrome(-121));
        assert_eq!(false, Solution::is_palindrome(10));
    }
}
