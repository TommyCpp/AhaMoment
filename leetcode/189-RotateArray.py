class Solution:
    def rotate(self, nums, k):
        """
        :type nums: List[int]
        :type k: int
        :rtype: void Do not return anything, modify nums in-place instead.
        """
        if k == 0:
            return
        if k > len(nums):
            k = k % len(nums)
            self.rotate(nums,k)
        else:
            temp = nums[-k:]
            temp += nums[0:len(nums)-k]
            nums[:] = temp[:]
