class Solution {
    public int[] nextGreaterElement(int[] nums1, int[] nums2) {
        HashMap<Integer,Integer> res = new HashMap();
        Stack<Integer> stack = new Stack();
        for(int n : nums2){
            while(!stack.empty() && n> stack.peek()){
                res.put(stack.pop(),n);
            }
            stack.push(n);
        }
        
        for(int i = 0;i < nums1.length; i++){
            nums1[i] = res.getOrDefault(nums1[i], -1);
        }
        return nums1;
    }
}
