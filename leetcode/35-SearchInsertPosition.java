class Solution {
    public int searchInsert(int[] nums, int target) {
        if(nums.length < 1 || nums[0] > target){ // if target should be at front of the array.
            return 0;
        }
        if(nums[nums.length - 1] < target){ // if target should be at end of the array.
            return nums.length;
        }
        int start = 0;
        int end = nums.length - 1;
        while(start <= end){
            int median = start + (end - start) / 2;
            if(nums[median] == target){
                return median;
            }
            if(nums[median] < target && nums[median + 1] > target){
                return median + 1;
            }
            if(nums[median] < target){
                start = median + 1;
            }
            if(nums[median] > target){
                end = median - 1;
            }
        }
        return 0;
    }
}
