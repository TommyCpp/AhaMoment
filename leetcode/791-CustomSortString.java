import java.util.Hashtable; 
class Solution {
    public String customSortString(String S, String T) {
        StringBuffer result = new StringBuffer(T.length());
        StringBuffer irrelavent = new StringBuffer();
        Hashtable<Character,Integer> table = new Hashtable<Character,Integer>();
        for(int i = 0;i < S.length(); i++){
            table.put(S.charAt(i),0);
        }
        for(int i = 0;i < T.length(); i++){
            Character ch = T.charAt(i);
            if(table.containsKey(ch)){
                table.put(ch, table.get(ch)+1);
            }
            else{
                irrelavent.append(ch);
            }
        }
        for(int i = 0;i < S.length(); i++){
                Integer chCount = table.get(S.charAt(i));
                for(int k = 0; k < chCount; k++){
                    result.append(S.charAt(i));
                }
        }
        return result.toString()+irrelavent.toString();
    }
}
