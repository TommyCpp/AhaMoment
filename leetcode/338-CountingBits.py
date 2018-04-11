class Solution:
    def countBits(self, num):
        """
        :type num: int
        :rtype: List[int]
        """
        result = [-1]*(num + 1)
        result[0] = 0
        for i in range(num + 1):
            if i == 0:
                result[i] = 0
            else:
                log = math.log2(i)
                if log.is_integer():
                    result[i] = 1
                else:
                    result[i] = result[i - 2**int(log)] + 1
        return result
