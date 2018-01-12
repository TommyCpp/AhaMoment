class Solution:
    def climbStairs(self, n):
        """
        :type n: int
        :rtype: int
        """
        if n == 0:
            return 0
        if n == 1:
            return 1
        d = [0]*n
        d[0] = 1
        d[1] = 2
        if n >= 2:
            for i in range(2,n):
                d[i] = d[i-1]+d[i-2]
    
        return d[n-1]
