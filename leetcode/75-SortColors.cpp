class Solution {
public:
    void sortColors(vector<int>& nums) {
        int red = 0;
        int white = 0;
        int blue = 0;
        for(int i:nums){
            if(i == 0){
                red ++;
            }
            if(i == 1){
                white++;
            }
            if(i == 2){
                blue++;
            }
        }
        int size = nums.size();
        for(int i=0;i<red;i++){
            nums[i]=0;
        }
        for(int i=red;i<white+red;i++){
            nums[i]=1;
        }
        for(int i=white+red;i<size;i++){
            nums[i]=2;
        }
    }
};
