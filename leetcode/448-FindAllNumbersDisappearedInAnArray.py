class Solution:
    def findDisappearedNumbers(self, nums):
        """
        :type nums: List[int]
        :rtype: List[int]
        """
        for n in range(len(nums)):
            m = abs(nums[n])
            if nums[m - 1] > 0:
                nums[m - 1] = -nums[m - 1]
        
        i = 1
        res = []
        for j in range(len(nums)):
            if nums[j] > 0:
                res.append(i)
            i += 1
        return res
