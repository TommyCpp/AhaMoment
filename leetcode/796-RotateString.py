class Solution:
    def rotateString(self, A, B):
        """
        :type A: str
        :type B: str
        :rtype: bool
        """
        if A == B:
            return True
        for i in range(len(A)):
            if B == A[i:] + A[:i]:
                return True
        return False
        
