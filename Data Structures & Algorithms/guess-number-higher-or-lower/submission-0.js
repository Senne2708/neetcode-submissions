/**
 * Forward declaration of guess API.
 * @param {number} num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * function guess(num) {}
 */

class Solution {
    /**
     * @param {number} n
     * @return {number}
     */
    guessNumber(n) {
        let low = 1;
        let high = n;
        let output = 1;

        while (output != 0) {
            let mid = (low + high) / 2;
            
            if (guess(mid) > 0) {
                low = mid + 1;
            } else if (guess(mid) < 0) {
                high = mid - 1;
            } else {
                return mid
            }
        }
    }
}
