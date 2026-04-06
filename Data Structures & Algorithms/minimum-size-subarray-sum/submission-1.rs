use std::cmp::min;
use std::i32::MAX;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut l_idx = 0;
        let mut running_total = 0;
        let mut len = MAX;

        for (r_idx, r_val) in nums.iter().enumerate() {
            running_total += r_val;

            while running_total >= target {
                len = min(len, (r_idx - l_idx + 1) as i32);
                running_total -= nums[l_idx];
                l_idx+=1;
            }
        }
        if (len == MAX) {
            return 0;
        }
        len
    }
}
