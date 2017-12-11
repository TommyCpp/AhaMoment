class Solution {
    public int findComplement(int num) {
        if(num<0){
            return -num-1;
        }
        else{
        /*
        Integer.highestOneBit(int a);
        find the leftmost 1 in bit set. 
        return the integer repersent.
        e.g   Integer.highestOneBit(10)  => 8
        */
            int hOne = Integer.highestOneBit(num)*2-1; // find the leftmost bit. and use 1 to fill the right, use 0 to fill the left.
            return (~num)&hOne;
        }
    }
}
