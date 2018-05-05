class Solution {
    public int numberOfArithmeticSlices(int[] A) {
        int[][] dp = new int[A.length][A.length];
        for(int i = 0; i < A.length - 2; i++){
            dp[i][i + 2] = (A[i + 1] - A[i]) == (A[i + 2] - A[i + 1]) ? 1 : 0;
            for(int j = i + 3;j < A.length; j++){
                dp[i][j] = (dp[i][j - 1] == 1) && ((A[j] - A[j - 1]) == (A[j - 1] - A[j - 2])) ? 1 : 0;
            }
        }
        int res  = 0;
        for(int i = 0; i < A.length; i++){
            for(int j = 0; j< A.length; j++){
                if(dp[i][j] == 1){
                    res ++;
                }
            }
        }
        return res;
    }
}
