import collections
class Solution:
    def findTheDifference(self, s, t):
        """
        :type s: str
        :type t: str
        :rtype: str
        """
        sd = collections.Counter(s)
        td = collections.Counter(t)
        for k, v in td.items():
            if k not in sd.keys():
                return k
            else:
                if v != sd[k]:
                    return k
