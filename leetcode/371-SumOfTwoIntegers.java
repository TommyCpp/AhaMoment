class Solution {
    public int getSum(int a, int b) {
        if(b == 0){
            return a;
        }
        int next = a & b;
        next <<= 1;
        a  = a ^ b;
        return getSum(a,next);
    }
}
