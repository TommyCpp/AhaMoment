class Solution {
    public int calPoints(String[] ops) {
        Stack<Integer> stack = new Stack();
        Integer res = 0;
        for(String i:ops){
            if(i.equals("C")){
                stack.pop();
            }
            else if(i.equals("D")){
                stack.push(stack.peek()*2);
            }
            else if(i.equals("+")){
                int i_1 = stack.pop();
                int i_2 = stack.pop();
                stack.push(i_2);
                stack.push(i_1);
                stack.push(i_1 + i_2);
            }
            else{
                stack.push(Integer.valueOf(i));
            }
        }
        while(stack.size()>0){
            res += stack.pop();
        }
        return res;
        
    }
}
