class Solution:        
    def convertToBase7(self, num):
        """
        :type num: int
        :rtype: str
        """
        if num >= 0:
            result = []
            num = num
        else:
            result = ["-"]
            num = -num
        while 1:
            temp = int(num / 7)
            if temp >= 7:
                result.append(self.convertToBase7(temp))
            else :
                if temp == 0:
                    result.append(str(num))
                    break
                else:
                    result.append(str(temp))
            num = num % 7
        return "".join(result)
