class Solution {
    public int[][] matrixReshape(int[][] nums, int r, int c) {
        if(nums.length * nums[0].length != r * c){
            return nums;
        }
        else{
            int[][] result = new int[r][c];
            int oriR = nums.length;
            int oriC = nums[0].length;
            int index = 0;
            for(int i = 0;i < r;i++){
                for (int j = 0;j < c;j++){
                    result[i][j] = nums[index / oriC][index % oriC];
                    index ++;
                }
            }
            return result;
        }
    }
}
