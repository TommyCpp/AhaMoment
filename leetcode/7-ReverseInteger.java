class Solution {
    public int reverse(int x) {
        boolean flag = false;
        if(x < 0){
            flag = true;
            x = -x;
        }
        String s = String.valueOf(x);
        StringBuffer sb = new StringBuffer(s);
        s = sb.reverse().toString();
        try{
            Integer result = Integer.valueOf(s);
            if(flag){
                return -result;
            }
            else{
                return result;
            }
        }catch(Exception e){
            return 0;
        }
       
        
    }
}
