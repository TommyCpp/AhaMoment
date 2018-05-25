class Solution:
    def minMoves2(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        nums = sorted(nums)
        median = nums[len(nums) // 2]
        res = 0
        for i in nums:
            res += abs(i - median)
        return res
