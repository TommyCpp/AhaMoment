import collections
import math
class Solution:
    def numRabbits(self, answers):
        """
        :type answers: List[int]
        :rtype: int
        """
        counter = collections.Counter(answers)
        res = 0
        for answer, rabbit in counter.items():
            if rabbit % (answer + 1) != 0:
                res += (rabbit // (answer + 1) + 1) * (answer + 1)
            else: 
                res += rabbit
        return res
