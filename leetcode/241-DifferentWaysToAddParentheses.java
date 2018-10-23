class Solution {
    public List<Integer> diffWaysToCompute(String input) {
        char[] chars = input.toCharArray();
        List<Integer> res = helper(chars, 0, chars.length - 1);
        return res;
    }
    
    public List<Integer> helper(char[] chars, int start, int end){
        List<Integer> res = new ArrayList();
        boolean flag = true;
        for(int i = start; i <= end; i++){
            if(chars[i] == '-' || chars[i] == '+' || chars[i] == '*'){
                flag = false;
                List<Integer> left = helper(chars, start, i - 1);
                List<Integer> right = helper(chars, i + 1, end);
                switch(chars[i]){
                    case '-':
                        for(Integer x : left){
                            for(Integer y :right){
                                res.add(x - y);
                            }
                        }break;
                    case '+':
                        for(Integer x : left){
                            for(Integer y :right){
                                res.add(x + y);
                            }
                        }break;
                    case '*':
                        for(Integer x : left){
                            for(Integer y :right){
                                res.add(x * y);
                            }
                        }break;
                }
            }
        }
        if(flag){
            res.add(Integer.valueOf(new String(chars).substring(start, end + 1)));
        }
        return res;  
    }
}
