class Solution {
    public int mySqrt(int x) {
        int start = 1;
        int end = x;
        while(start <= end){
            int median = start + (end - start) / 2;
            if(Math.pow(median, 2) == x || Math.pow(median, 2) < x && Math.pow(median+1, 2) > x){
                return median;
            }
            if(Math.pow(median, 2) > x){
                end = median - 1;
            }
            else if(Math.pow(median, 2) < x){
                start = median + 1;
            }
        }
        return 0;
    }
}
