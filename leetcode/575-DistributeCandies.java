import java.util.Hashtable;
import java.util.HashSet;
import java.util.Arrays;
class Solution {
    public int distributeCandies(int[] candies) {
        Set<Integer> set = new HashSet();
        for(int candy:candies){
            set.add(candy);
            if(set.size() == candies.length / 2){
                return set.size();
            }
        }
        return set.size();
    }
}
