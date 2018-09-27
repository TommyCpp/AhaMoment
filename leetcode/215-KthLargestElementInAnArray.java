class Solution {
    public int findKthLargest(int[] nums, int k) {
        return findKthLargest(nums, k - 1, 0, nums.length - 1);
    }
    
    public int findKthLargest(int[] nums, int k ,int lo, int hi){
        if(hi == lo){
            return nums[lo];
        }
        int j = partition(nums, k, lo, hi);
        if(j == k){
            return nums[j];
        }
        else{
            if (j > k){
                return findKthLargest(nums, k, lo, j - 1);
            }
            else{
                return findKthLargest(nums, k, j + 1, hi);
            }
        }
    }
    
    public int partition(int[] nums, int k, int lo, int hi){
        int i = lo;
        int j = hi + 1;
        int v = nums[lo];
        while(true){
          while(nums[++i] > v) if(i == hi) break;
          while(nums[--j] < v) if(j == lo) break;
          if(i >= j) break;
          exch(nums, i, j);
        }
        exch(nums, lo, j);
        return j;
    }
    
    public void exch(int[] nums, int a,int b){
        int t = nums[a];
        nums[a] = nums[b];
        nums[b] = t;
    }
}
