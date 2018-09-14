class Solution {
    public boolean isPalindrome(String s) {
        StringBuffer buffer = new StringBuffer();
        for(int i = 0;i<s.length();i++){
            if(Character.isAlphabetic((int)s.charAt(i)) || Character.isDigit((int)s.charAt(i))){
                buffer.append(Character.toLowerCase(s.charAt(i)));
            }
        }
        String b = buffer.toString();
        String a = buffer.reverse().toString();
        return a.equals(b);
        
    }
}
