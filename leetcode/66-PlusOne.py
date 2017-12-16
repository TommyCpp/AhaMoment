class Solution:
    def plusOne(self, digits):
        """
        :type digits: List[int]
        :rtype: List[int]
        """
        plus = True
        for i in range(len(digits) - 1,-1,-1):
            if digits[i] == 9 and plus:
                digits[i] = 0
            else:
                if plus:
                    digits[i] += 1
                    plus = False
                    return digits
                else:
                    break
        if plus:
            digits.insert(0,1)
        return digits
