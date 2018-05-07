class Solution {
    public int findMaxConsecutiveOnes(int[] nums) {
        int res = 0;
        int i = 0;
        while(i < nums.length){
            if(nums[i] == 1){
                int j  = 0;
                while(i < nums.length && nums[i] != 0){
                    j ++;
                    i ++;
                }
                if(j > res){
                    res = j;
                }
                i ++;
            }
            else{
                while(i < nums.length && nums[i] == 0){
                    i++;
                }
            }
        }
        return res;
    }
}
