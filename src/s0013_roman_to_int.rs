//
// https://leetcode.com/problems/roman-to-integer/
//
// Roman numerals are represented by seven different symbols:
// I, V, X, L, C, D and M.
//
// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
//
// For example, 2 is written as II in Roman numeral, just two ones
// added together. 12 is written as XII, which is simply X + II.
// The number 27 is written as XXVII, which is XX + V + II.
//
// Roman numerals are usually written largest to smallest from left
// to right. However, the numeral for four is not IIII. Instead, the
// number four is written as IV. Because the one is before the five
// we subtract it making four. The same principle applies to the
// number nine, which is written as IX. There are six instances
// where subtraction is used:
//
// I can be placed before V (5) and X (10) to make 4 and 9.
// X can be placed before L (50) and C (100) to make 40 and 90.
// C can be placed before D (500) and M (1000) to make 400 and 900.
// Given a roman numeral, convert it to an integer.
//
//
// Example 1:
//
// Input: s = "III"
// Output: 3
// Explanation: III = 3.
//
//
// Example 2:
//
// Input: s = "LVIII"
// Output: 58
// Explanation: L = 50, V= 5, III = 3.
//
//
// Example 3:
//
// Input: s = "MCMXCIV"
// Output: 1994
// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
//
//
// Hint 1:
//
// Problem is simpler to solve by working the string from back to front and using a map.
//
//
// Symbol       Value
// M             1000
// CM            900
// D             500
// CD            400
// C             100
// XC            90
// L             50
// XL            40
// X             10
// IX            9
// V             5
// IV            4
// I             1
//

#[allow(dead_code)]

pub struct Solution;

// === submission begin ========================================

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let table: Vec<(i32, &'static str)> = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        let mut sum = 0;
        let mut idx = 0;

        for ptr in table.iter() {
            while idx + ptr.1.len() <= s.len() && ptr.1 == &s[idx..idx + ptr.1.len()] {
                idx += ptr.1.len();
                sum += ptr.0;
                if idx >= s.len() {
                    return sum;
                }
            }
        }
        sum
    }
}

// === submission end ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
    }

    #[test]
    fn example2() {
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
    }

    #[test]
    fn example3() {
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }
}
