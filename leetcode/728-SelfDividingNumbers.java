class Solution {
    //Note that when use Integer.valueOf(char) to try to convert char to number, it will return ascii value.
    //Should use char - '0' or Character.getNumericValue() instead.
    
    public List<Integer> selfDividingNumbers(int left, int right) {
        List<Integer> results = new ArrayList(right - left);
        for(int number = left; number <= right; number++){
            char[] digits = ("" + number).toCharArray();
            boolean flag = true;
            for(char digit: digits){
                if(digit == '0'){
                    flag = false;
                    break;
                }
                else{
                    if(number % (digit-'0') != 0){
                        flag = false;
                    }
                }
            }
            if(flag){
                results.add(number);
            }
        }
        return results;
    }
}
