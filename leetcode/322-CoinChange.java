class Solution {
    public int coinChange(int[] coins, int amount) {
        int[] dp = new int[amount + 1];
        dp[0] = 0;
        for(int i = 1; i < amount + 1; i++){
            dp[i] = -1;
            for(int j = 0; j < coins.length;j++){
                if(i == coins[j]){
                    dp[i] = 1;
                    break;
                }
                else if(i < coins[j]){
                    continue;
                }
                else if(dp[i - coins[j]] != -1){
                    if(dp[i] == -1 || dp[i - coins[j]] + 1 < dp[i]){
                        dp[i] = dp[i - coins[j]] + 1;
                    }
                }
            }
        }
        // for(int i = 0;i < dp.length;i++){
        //     System.out.println(dp[i]);
        // }
        return dp[amount];
        
    }
}
