use std::collections::HashSet;

impl Solution {
    // Brute force solution: O(n*k)
    // Optimised with hashset: O(n)
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut window_set = HashSet::new();
        let mut left = 0usize;

        // Right pointer iterates through vector
        for (right, n) in nums.iter().enumerate() {
            
            // Check size of window, remove left value 
            // and increment left pointer
            if right - left > k as usize {
                window_set.remove(&nums[left]);
                left += 1;
            }

            // Check if right value is already in the hashset (duplicate inside window)
            if window_set.contains(&n) {
                return true;
            }

            // Add right value to the set
            window_set.insert(n);
        }

        // No duplicates found
        false
    }
}
