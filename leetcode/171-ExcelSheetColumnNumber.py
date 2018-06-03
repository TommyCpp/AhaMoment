class Solution:
    def titleToNumber(self, s):
        """
        :type s: str
        :rtype: int
        """
        res = 0
        exp = 0
        for i in range(len(s) - 1, -1, -1):
            val = ord(s[i])- ord('A') + 1
            res += int(val * (26**exp))
            exp += 1
        return res
