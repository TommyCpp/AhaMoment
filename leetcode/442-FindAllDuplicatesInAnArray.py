class Solution:
    def findDuplicates(self, nums):
        """
        :type nums: List[int]
        :rtype: List[int]
        """
        nums.sort()
        for i in range(len(nums)):
            if i != 0:
                if nums[i] == nums[i-1]:
                    nums[i] = -nums[i]
        nums = list(filter(lambda x: x < 0, nums))
        for i in range(len(nums)):
            nums[i] = -nums[i]
        return nums
