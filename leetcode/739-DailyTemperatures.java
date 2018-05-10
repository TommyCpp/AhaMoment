class Solution {
    public int[] dailyTemperatures(int[] temperatures) {
        Stack<int[]> stack = new Stack();
        int[] res = new int[temperatures.length];
        for(int i = 0; i < temperatures.length; i++){
            if(stack.empty()){
                stack.push(new int[]{i,temperatures[i]});
            }
            else{
                while(!stack.empty() && stack.peek()[1] < temperatures[i]){
                    int[] item = stack.pop();
                    res[item[0]] = i - item[0];
                }
                stack.push(new int[]{i,temperatures[i]});
            }
        }
        while(stack.empty()){
            res[stack.pop()[0]] = 0;
        }
        return res;
    }
}
