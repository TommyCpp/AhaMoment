class Solution {
    public List<String> fizzBuzz(int n) {
        String[] arr = new String[n];
        for (int i = 0, j = 1; i < n; i++, j++) {
            if(j % 15 == 0) 
                arr[i] = "FizzBuzz";
            else if (j %  3 == 0) 
                arr[i] = "Fizz";
            else if (j %  5 == 0) 
                arr[i] = "Buzz";
            else                  
                arr[i] = String.valueOf(j);
        }
        return Arrays.asList(arr);
    }
}
