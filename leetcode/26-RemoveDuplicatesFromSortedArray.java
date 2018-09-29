class Solution {
    public int removeDuplicates(int[] nums) {
        if(nums.length <= 1){
            return nums.length;
        }
        int index = 0;
        for(int i = 0;i < nums.length - 1; i++){
            if(nums[i] == nums[i + 1]){
                continue;
            }
            else{
                nums[index] = nums[i];
                index ++;
            }
        }
        if(index == 0){
            nums[0] = nums[nums.length - 1];
            return 1;
        }
        if(nums[nums.length - 1] != nums[index - 1]){
            nums[index] = nums[nums.length - 1];
        }
        return index + 1;
    }
}
