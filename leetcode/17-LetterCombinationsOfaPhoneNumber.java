class Solution {
    public List<String> letterCombinations(String digits) {
        if(digits.length() == 0){
            return new ArrayList();
        }
        HashMap<Character, String> hashMap = new HashMap();
        hashMap.put('2', "abc");
        hashMap.put('3', "def");
        hashMap.put('4', "ghi");
        hashMap.put('5', "jkl");
        hashMap.put('6', "mno");
        hashMap.put('7', "pqrs");
        hashMap.put('8', "tuv");
        hashMap.put('9', "wxyz");
        List<String> res = new ArrayList();
        List<String> recRes = this.letterCombinations(digits.substring(1));
        for(char ch: hashMap.get(digits.charAt(0)).toCharArray()){
            if(recRes.size() == 0){
                res.add(String.valueOf(ch));
            }
            else{
                for(String str: recRes){
                    res.add(String.valueOf(ch) + str);
                }
            }

        }
        return res;
        
    }
}
