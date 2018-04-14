class Solution {
    public String[] findWords(String[] words) {
        List<String> list = new ArrayList();
        for(String word: words){
            int row = -1;
            boolean flag = true;
            String lowerWord = word.toLowerCase();
            for(int i=0;i<word.length();i++){
                Character ch = lowerWord.charAt(i);
                if(row == -1){
                    
                    if("qwertyuiop".indexOf(ch)!= -1){
                        row = 0;
                    }
                    else if("asdfghjkl".indexOf(ch)!=-1){
                        row = 1;
                    }
                    else{
                        row = 2;
                    }
                }
                else{
                    if(row == 0){
                        if("qwertyuiop".indexOf(ch) == -1){
                            flag = false;
                        }   
                    }
                    else if(row == 1){
                        if("asdfghjkl".indexOf(ch) == -1){
                            flag = false;
                        }
                    }
                    else{
                        if("zxcvbnm".indexOf(ch) == -1){
                            flag = false;
                        }
                    }
                }
            }
            if(flag){
                list.add(word);
            }
        }
        String[] result = new String[list.size()];
        list.toArray(result);
        return result;
    }
}
