class Solution {
    public int singleNumber(int[] nums) {
        int a = 0;
        int b = 0;
        for(int i = 0;i < nums.length; i++){
            b = b ^ nums[i] & ~a;
            a = a ^ nums[i] & ~b;
        }
        return a | b;
    }
}
