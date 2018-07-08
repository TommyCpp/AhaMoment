class Solution:
    def countBinarySubstrings(self, s):
        """
        :type s: str
        :rtype: int
        """
        res = 0
        i = 1
        cur = 1
        last = 0
        for i in range(1, len(s)):
            if s[i] == s[i - 1]:
                cur += 1
            else:
                last = cur
                cur = 1
                
            if last >= cur:
                res += 1
        return res
                
