class Solution {
    public int search(int[] nums, int target) {
        int start = 0;
        int end = nums.length - 1;
        while(start <= end){
            int median = start + (end - start) / 2;
            if(nums[median] > target){
                end = median - 1;
            }
            else if(nums[median] < target){
                start = median + 1;
            }
            else{
                return median;
            }            
        }
        
        return -1;
    }
}
