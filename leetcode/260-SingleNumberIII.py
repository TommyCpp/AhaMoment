# Solution 1, use Counter API
import collections
class Solution:
    def singleNumber(self, nums):
        """
        :type nums: List[int]
        :rtype: List[int]
        """
        counter = collections.Counter(nums)
        return [ i[0] for i in counter.most_common() if i[1] == 1]
        

# Solution 2, use bit manipulation       
import collections
class Solution:
    def singleNumber(self, nums):
        """
        :type nums: List[int]
        :rtype: List[int]
        """
        diff = 0
        for i in nums:
            diff ^= i
        diff &= -diff
        res_a = 0
        res_b = 0
        for i in nums:
            if diff & i == 0:
                res_a ^= i
            else:
                res_b ^= i
        return [res_a,res_b]
