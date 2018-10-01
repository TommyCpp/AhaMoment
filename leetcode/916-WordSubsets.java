class Solution {
    public List<String> wordSubsets(String[] A, String[] B) {
        int[] chars = new int[26];
        for(String s: B){
            int[] charInStr = new int[26];
            for(int i = 0;i < s.length(); i++){
                charInStr[s.charAt(i) - 97] += 1;
            }
            for(int i = 0;i < 26; i++){
                if(charInStr[i] > chars[i]){
                    chars[i] = charInStr[i];
                }
            }
        }
        List<String> res = new ArrayList();
        for(String s : A){
            int[] charInStr = new int[26];
            boolean flag = true;
            for(int i = 0;i < s.length(); i++){
                charInStr[s.charAt(i) - 97] += 1;
            }
            for(int i = 0;i < 26; i++){
                if(charInStr[i] < chars[i]){
                    flag = false;
                }
            }
            if(flag){
                res.add(s);
            }
        }
        return res;
    }
}
