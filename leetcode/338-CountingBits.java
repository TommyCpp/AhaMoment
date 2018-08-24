class Solution {
    public int[] countBits(int num) {
        if(num == 0){
            return new int[]{0};
        }
        int[] res = new int[num + 1];
        res[0] = 0;
        int prePower = 0;
        for(int i = 1;i < res.length; i++){
            if((i & (i - 1)) == 0){
                // if is 2's pow
                res[i] = 1;
                prePower = i;
                
            }
            else{
                res[i] = res[i - prePower] + 1;
            }
        }
        return res;
    }
}
