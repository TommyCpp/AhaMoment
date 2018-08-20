class Solution {
    public int[] nextGreaterElements(int[] nums) {
        if(nums.length == 0){
            return new int[0];
        }
        Stack<Integer> valStack = new Stack();
        Stack<Integer> indexStack = new Stack();
        valStack.add(nums[0]);
        indexStack.add(0);
        int[] res = new int[nums.length];
        for(int i = 0; i < nums.length;i++){
            res[i] = -1;
        }
        for(int i = 1;i < nums.length;i++){
                while(!valStack.isEmpty() && valStack.peek() < nums[i]){
                    valStack.pop();
                    res[indexStack.pop()] = nums[i];
                }
                valStack.add(nums[i]);
                indexStack.add(i);
        }
        for(int i = 0;i < nums.length;i++){
            while(nums[i] > valStack.peek()){
                valStack.pop();
                res[indexStack.pop()] = nums[i];
            }
        }
        return res;
    }
}
