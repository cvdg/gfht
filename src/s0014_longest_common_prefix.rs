//
// https://leetcode.com/problems/longest-common-prefix/
//
// Write a function to find the longest common prefix string
// amongst an array of strings.
//
// If there is no common prefix, return an empty string "".
//
//
// Example 1:
//
// Input: strs = ["flower","flow","flight"]
// Output: "fl"
//
//
// Example 2:
//
// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.
//
//

#[allow(dead_code)]

pub struct Solution;

// === submission begin ========================================

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = String::new();

        // loop over all charactes in the first string
        for (i1, c1) in strs[0].char_indices() {
            // loop over strings from the second
            for s in &strs[1..] {
                // string shorter
                if s.len() <= i1 {
                    return prefix;
                } else {
                    let c2 = s.chars().nth(i1).unwrap();
                    // different character
                    if c1 != c2 {
                        return prefix;
                    }
                }
            }

            prefix.push(c1);
        }
        prefix
    }
}

// === submission end ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            "fl",
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            "",
            Solution::longest_common_prefix(vec![
                "".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ])
        );
    }
}
