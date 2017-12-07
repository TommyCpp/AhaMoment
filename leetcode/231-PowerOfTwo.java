class Solution {
    public boolean isPowerOfTwo(int n) {
        if(n <= 0) return false;
        int num_1 = 0;
        while(n != 0){
            num_1 += n & 1; //Add 1 or zero
            n >>= 1;
        }
        return num_1 == 1;
    }
}
