class Solution {
    public int peakIndexInMountainArray(int[] A) {
        int max = Integer.MIN_VALUE;
        int index = -1;
        for(int i = 1;i < A.length;i++){
            if(A[i] > max){
                max = A[i];
                index = i;
            }
        }
        return index;
    }
}
