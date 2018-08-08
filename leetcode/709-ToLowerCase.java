class Solution {
    public String toLowerCase(String str) {
        String result = "";
        for(char ch: str.toCharArray()){
            if(ch > 64 && ch < 91){
                result += (char)(ch + 32);
            }
            else{
                result += ch;
            }
        }
        return result;
    }
}
