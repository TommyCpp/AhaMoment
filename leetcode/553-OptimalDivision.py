class Solution:
    def optimalDivision(self, nums):
        """
        :type nums: List[int]
        :rtype: str
        """
        return str(nums[0]) + ("/(" + "/".join([str(i) for i in nums[1:]]) +")" if len(nums) > 2 else ("/" + str(nums[1]) if len(nums) > 1 else ""))
