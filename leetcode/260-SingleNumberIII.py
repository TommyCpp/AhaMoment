import collections
class Solution:
    def singleNumber(self, nums):
        """
        :type nums: List[int]
        :rtype: List[int]
        """
        counter = collections.Counter(nums)
        return [ i[0] for i in counter.most_common() if i[1] == 1]
        
