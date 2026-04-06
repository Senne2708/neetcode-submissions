impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let window_size = k as usize;
        let total_threshold: i64 = (threshold as i64) * (k as i64);

        let mut running_total: i64 = 0i64;
        let mut total = 0;

        for right in 0..arr.len() {
            running_total += arr[right] as i64; 

            if right >= window_size {
                running_total -= arr[right - window_size] as i64;
            }

            if (right >= window_size - 1) && (running_total >= total_threshold) {
                total += 1;
            } 
        }

        total
    }
}
