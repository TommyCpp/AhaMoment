class Solution:
    def lemonadeChange(self, bills):
        """
        :type bills: List[int]
        :rtype: bool
        """
        five = 0
        ten = 0
        twenty = 0
        for bill in bills:
            if bill == 5:
                five += 1
            elif bill == 10:
                if five < 1:
                    return False
                else:
                    five -= 1
                    ten += 1
            else:
                if five > 0 and ten > 0:
                    five -= 1
                    ten -= 1
                    twenty += 1
                    continue 
                if five > 2:
                    five -= 3
                    twenty += 1
                    continue
                else:
                    return False
                    
        return True
