public class Solution {
    public String reverseWords(String s) {
        String[] strs = s.split(" ");
        StringBuilder sb = new StringBuilder();
        for(int i = strs.length - 1; i >= 0;i--){
            if(strs[i].isEmpty())
                continue;
            if(i == strs.length - 1){
                sb.append(strs[i]);
            }
            else{
                sb.append(" " + strs[i]);
            }
        }
        return sb.toString().trim();
    }
}
