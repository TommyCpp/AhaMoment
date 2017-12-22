class MinStack:

    def __init__(self):
        """
        initialize your data structure here.
        """
        self.data = []
        self.minData = []
        

    def push(self, x):
        """
        :type x: int
        :rtype: void
        """
        self.data.insert(0,x)
        if len(self.minData) == 0 or x <= self.minData[0]:
            self.minData.insert(0,x)
        

    def pop(self):
        """
        :rtype: void
        """
        if self.data[0] == self.minData[0]:
            result = self.data[0]
            self.data = self.data[1:]
            self.minData = self.minData[1:]
            return result
        else:
            result = self.data[0]
            self.data = self.data[1:]
            return result

    def top(self):
        """
        :rtype: int
        """
        return self.data[0]
        

    def getMin(self):
        """
        :rtype: int
        """
        return self.minData[0]
        
