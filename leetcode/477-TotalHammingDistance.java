class Solution {
    public int totalHammingDistance(int[] nums) {
        int res = 0;
        int zeros = 0;
        int ones = 0;
        while(true){
            zeros = 0;
            ones = 0;
            int countZero = 0;
            for(int i = 0;i < nums.length;i++){
                if(nums[i] == 0){
                    countZero += 1;
                    zeros += 1;
                    continue;
                }
                if((nums[i] & 1) == 1){
                    ones += 1;
                }
                else{
                    zeros += 1;
                }
                nums[i] >>= 1;
            }
            
            if(countZero < nums.length){
                if(zeros != 0 && ones != 0){
                    res += zeros*ones;
                }   
            }
            else{
                break;
            }
        }
        return res;
    }
}
