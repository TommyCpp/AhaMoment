class Solution {
    public boolean isPalindrome(int x) {
        if(x < 0){
            return false;
        }
        else{
            List<Integer> nums = new ArrayList();
            while(x > 0){
                nums.add(x % 10);
                x = x / 10;
            }
            Object[] numbers = nums.toArray();
            for(int i = 0, j = numbers.length - 1; i < j;i ++,j--){
                if((int)numbers[i] != (int)numbers[j]){
                    returan false;
                }
            }
            return true;
        }
    }
}
