class Solution:
    def numJewelsInStones(self, J, S):
        """
        :type J: str
        :type S: str
        :rtype: int
        """
        """
        By HashTable, we can control the iterator J and S only one time.
        """
        counter = {}
        for s in S:
            if s in counter.keys():
                counter[s] += 1
            else:
                counter[s] = 1
        result = 0
        for j in J:
            result += counter.get(j,0)
        return result
        
