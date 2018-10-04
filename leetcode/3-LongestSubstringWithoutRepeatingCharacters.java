class Solution {
    public int lengthOfLongestSubstring(String s) {
        if(s.length() == 1) return 1;
        Set<Character> set = new HashSet();
        int i = 0, j = 0, res = 0;
        while(i < s.length() && j < s.length()){
            if(!set.contains(s.charAt(j))){
                set.add(s.charAt(j++));
            }
            else{
                if(j - i > res){
                    res = j - i;
                }
                set.remove(s.charAt(i++));
            }
        }
        if(j - i > res){
            res = j - i;
        }
        return res;
    }
}
