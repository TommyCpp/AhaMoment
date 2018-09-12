class Solution {
    public String longestCommonPrefix(String[] strs) {
        if(strs.length == 0){
            return "";
        }
        if(strs.length == 1){
            return strs[0];
        }
        Arrays.sort(strs);
        String a = strs[0];
        String b = strs[strs.length - 1];
        StringBuffer res = new StringBuffer();
        for(int i = 0;i < Math.min(a.length(), b.length());i++){
            if(a.charAt(i) == b.charAt(i)){
                res.append(a.charAt(i));
            }
            else{
                break;
            }
        }
        return res.toString();
    }
}
