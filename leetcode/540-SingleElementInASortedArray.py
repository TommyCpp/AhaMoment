class Solution:
    def singleNonDuplicate(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        if len(nums) < 3:
            return None
        if nums[0] != nums[1]:
            return nums[0]
        
        i = 2
        while i < len(nums) - 2:
            if nums[i] != nums[i+1]:
                return nums[i]
            else:
                i += 2
        return nums[i]
