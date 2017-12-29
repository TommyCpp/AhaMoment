class Solution:
    def reverseString(self, s):
        """
        :type s: str
        :rtype: str
        """
        length = len(s)-1
        s = list(s)
        for i in range(int(len(s)/2)):
            t = s[i]
            s[i] = s[length - i]
            s[length - i] = t
        return "".join(s)
