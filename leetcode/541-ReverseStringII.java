class Solution {
    public String reverseStr(String s, int k) {
        if(s.length() < k){
            return new StringBuilder(s).reverse().toString();
        }
        StringBuilder sb = new StringBuilder();
        for(int i = 0;i < s.length();i+=2*k){
            for(int j = i + k - 1 > s.length() ? s.length() - 1: i + k - 1; j >= i;j --){
                sb.append(s.charAt(j));
            }
            for(int j = i + k;j < i + 2*k && j < s.length(); j++){
                sb.append(s.charAt(j));
            }
            if(i >= s.length()){
                break;
            }
        }
        return sb.toString();
    }
}
