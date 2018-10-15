class Solution {
    public int findRadius(int[] houses, int[] heaters) {
        Arrays.sort(houses);
        Arrays.sort(heaters);
        int i = 0;
        int j = houses.length - 1;
        int t = 0;
        int res = 0;
        while(i < houses.length && houses[i] < heaters[0]){
            res = res < Math.abs(houses[i] - heaters[0])? Math.abs(houses[i] - heaters[0]) : res;
            i++;
        }
        while(j > -1 && houses[j] > heaters[heaters.length - 1]){
            res = res < Math.abs(houses[j] - heaters[heaters.length - 1])? Math.abs(houses[j] - heaters[heaters.length - 1]) : res;
            j --;
        }
        while(i <= j && t < heaters.length - 1){
            while(t < heaters.length - 2 && !(heaters[t] <= houses[i] && heaters[t + 1] >= houses[i])){
                t++;
            }

            res = res < Math.min(Math.abs(houses[i] - heaters[t]), Math.abs(houses[i] - heaters[t + 1]))? Math.min(Math.abs(houses[i] - heaters[t]), Math.abs(houses[i] - heaters[t + 1])) : res ;
            i ++;
        }
        return res;
    }
}
