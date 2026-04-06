class Solution {
    /**
     * @param {number[]} nums
     * @return {void} Do not return anything, modify nums in-place instead.
     */
    sortColors(nums) {
        let count = [0, 0, 0];

        for (let i = 0; i < nums.length; i++) {
            let pos = nums[i]
            count[pos] += 1;
        }

        let index = 0;
        for (let color = 0; color < 3; color++) {
            for (let i = 0; i < count[color]; i++) {
                nums[index++] = color;
            }
        }
    }
}
