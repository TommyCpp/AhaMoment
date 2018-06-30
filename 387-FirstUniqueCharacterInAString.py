import collections
class Solution:
    def firstUniqChar(self, s):
        """
        :type s: str
        :rtype: int
        """
        res = len(s)
        counter = collections.Counter(s)
        elements = counter.most_common()
        for i in range(len(elements)-1, -1,-1):
            if elements[i][1] == 1:
                index = s.index(elements[i][0])
                if index < res:
                    res = index
        if res == len(s):
            return -1
        else:
            return res
