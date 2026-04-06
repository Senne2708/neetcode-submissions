impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut global_max = nums[0];
        let mut global_min = nums[0];
    
        let mut cur_max = 0;
        let mut cur_min = 0;
        
        let mut total = 0;
    
        for &n in nums.iter() {
            cur_max = max(cur_max + n, n);
            cur_min = min(cur_min + n, n);
            total += n;
            global_max = max(global_max, cur_max);
            global_min = min(global_min, cur_min);
        }
    
        if global_max > 0 {
            max(global_max, total - global_min)
        } else {
            global_max
        } 
    }
}
