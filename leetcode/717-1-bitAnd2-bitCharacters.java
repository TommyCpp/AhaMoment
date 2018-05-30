class Solution {
    public boolean isOneBitCharacter(int[] bits) {
        boolean flag = false;
        for(int i = 0; i < bits.length; i++){
            if(bits[i] == 1){
                i ++;
                flag = false;
            }
            else{
                flag = true;
            }
        }
        return flag;
    }
}
