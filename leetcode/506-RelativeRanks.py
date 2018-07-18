class Solution:
    def findRelativeRanks(self, nums):
        """
        :type nums: List[int]
        :rtype: List[str]
        """
        nums = sorted(list(enumerate(nums)),key=lambda x:x[1], reverse=True)
        res = [-1]*len(nums)
        rank = 1
        for i, j in nums:
            if rank == 1:
                res[i] = "Gold Medal"
            elif rank == 2:
                res[i] = "Silver Medal"
            elif rank == 3:
                res[i] = "Bronze Medal"
            else:
                res[i] = str(rank)
            rank += 1
        return res
