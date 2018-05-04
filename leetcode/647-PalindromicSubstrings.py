class Solution:
    def countSubstrings(self, s):
        """
        :type s: str
        :rtype: int
        """
        dp = []
        for i in range(len(s)):
            dp.append([-1]*len(s))
        for i in range(len(s)):
            for j in range(i,len(s)):   
                isPalindromic(s, dp, i, j)
        res = 0
        for i in range(len(s)):
            for j in range(i, len(s)):
                if dp[i][j] == 1:
                    res += 1
        # print(dp)
        return res

def isPalindromic(s, dp, i, j):
    if i == j:
        dp[i][j] = 1
    else:
        if i == j - 1:
            dp[i][j] = 1 if s[i] == s[j] else 0
        else:
            if dp[i + 1][j - 1] == -1:
                isPalindromic(s, dp, i + 1, j - 1)
            dp[i][j] = 1 if dp[i + 1][j - 1] and s[i] == s[j] else 0
        
        
        
