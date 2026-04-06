use std::cmp::max;
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let characters: Vec<char> = s.chars().collect();
        let mut char_set = HashSet::new(); // Hashset that contains characters in current subarray
        let mut l_idx = 0usize;
        let mut total = 0i32;

        for (r_idx, r_val) in characters.iter().enumerate() {
            while char_set.contains(r_val) { 
                char_set.remove(&characters[l_idx]);
                l_idx += 1;
            }

            char_set.insert(r_val);
            total = max(total, (r_idx - l_idx + 1) as i32)
        }

        total
    }
}
