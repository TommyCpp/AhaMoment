class Solution {
    public String convert(String s, int numRows) {
        if(numRows == 1){
            return s;
        }
        StringBuffer[] zig = new StringBuffer[numRows];
        for(int i = 0; i < numRows; i++){
            zig[i] = new StringBuffer();
        }
        for(int i = 0;i < s.length(); i++){
            int j = i % (2*numRows - 2);
            if(j < numRows){
                zig[j].append(s.charAt(i));
            }
            else{
                zig[2*numRows - j - 2].append(s.charAt(i));
            }
        }
        StringBuffer res = new StringBuffer();
        for(StringBuffer buffer: zig){
            res.append(buffer);
        }
        return res.toString();
    }
}
