class Solution {
    public int minDistance(String word1, String word2) {
        if(word1.length() == 0){
            return word2.length();
        }
        if(word2.length() == 0){
            return word1.length();
        }
        int[][] res = new int[word1.length() + 1][word2.length() + 1];
        res[0][0] = 0;
        for(int i = 1;i < word1.length() + 1; i++){
            res[i][0] = i;
        }
        for(int i = 1;i < word2.length() + 1; i++){
            res[0][i] = i;
        }
        for(int i = 1; i < word1.length() + 1; i++){
            for(int j = 1;j < word2.length() + 1; j++){
                if(word1.charAt(i - 1) == word2.charAt(j - 1)){
                    res[i][j] = res[i - 1][j - 1];
                }
                else{
                   res[i][j] = Math.min(Math.min(res[i][j - 1] + 1, res[i - 1][j] + 1), res[i - 1][j - 1] + 1); 
                }
            }
        }
        return res[word1.length()][word2.length()];
    }
}
