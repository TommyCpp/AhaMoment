class Solution {
    
    public List<String> generateParenthesis(int n) {
        List<String> res = new ArrayList();
        helper(n,n,"",res);
        return res;
    }
    
    public void helper(int left, int right, String cur, List res){
        if(left == 0 && right == 0){
            res.add(cur);
        }
        else{
            if(right < left || left < 0 || right < 0){
                return ;
            }
            if(right >= left){
                helper(left - 1, right, cur+"(", res);
                helper(left, right - 1, cur+")", res);
            }
        }
    }
}
