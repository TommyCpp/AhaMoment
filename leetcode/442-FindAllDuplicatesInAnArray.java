class Solution {
    public List<Integer> findDuplicates(int[] nums) {
        List<Integer> res = new ArrayList();
        for(int i = 0;i < nums.length;i++){
            int visitedIndex = Math.abs(nums[i]) - 1;
            if(nums[visitedIndex] < 0){
                res.add(Math.abs(nums[i]));
            }
            else{
                nums[visitedIndex] = -nums[visitedIndex];
            }
        }
        return res;
    }
}
