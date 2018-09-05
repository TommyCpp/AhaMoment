class Solution {
    public boolean isValid(String s) {
        Stack<Character> stack = new Stack<Character>();
        if(s.length() == 0){
            return true;
        }
        for(int i = 0; i < s.length();i++){
            char ch = s.charAt(i);
            if(stack.isEmpty()){
                stack.push(ch);
            }
            else{
                char symbol = stack.peek();
                if((symbol == '(' && ch == ')')|| (symbol == '[' && ch == ']') || (symbol == '{' && ch == '}')){
                    stack.pop();
                    continue;
                }
                else{
                    stack.push(ch);
                }
            }
        }
        return stack.isEmpty();
    }
}
