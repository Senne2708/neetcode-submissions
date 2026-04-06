use std::cmp;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut maxSum = nums[0];
        let mut curSum = 0; 

        for n in nums.iter() {
            // 1. curSum must not be negative
            curSum = cmp::max(curSum, 0);
            // 2. Add current number to current sum 
            curSum += n;
            // 3. Check if the current sum is greater tham the max sum
            maxSum = cmp::max(maxSum, curSum);
        }

        maxSum
    }
}
