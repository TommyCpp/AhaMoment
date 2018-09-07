class Solution {
    public int pivotIndex(int[] nums) {
        if(nums.length == 0){
            return -1;
        }
        int[] left  = new int[nums.length];
        int sum = 0;
        for(int i = 0; i < nums.length; i++){
            sum += nums[i];
            left[i] = sum;
        }
        for(int i = 0;i < nums.length; i++){
            if(left[i] - nums[i] == sum - left[i]){
                return i;
            } 
        }
        return -1;
    }
}
