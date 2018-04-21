class Solution:
    def isToeplitzMatrix(self, matrix):
        """
        :type matrix: List[List[int]]
        :rtype: bool
        """
        rank = len(matrix) + len(matrix[0]) - 1
        for i in range(len(matrix)):
            num =  matrix[i][0]
            j = i + 1
            m = 1
            flag = True
            while j < len(matrix) and m < len(matrix[0]):
                if matrix[j][m] != num:
                    flag = False
                    break
                j += 1
                m += 1
            if not flag:
                return False
        
        if len(matrix[0]) > 1:
            for i in range(1,len(matrix[0])):
                num =  matrix[0][i]
                j = i + 1
                m = 1
                flag = True
                while m < len(matrix) and j < len(matrix[0]):
                    if matrix[m][j] != num:
                        flag = False
                        print(matrix[m][j])
                        break
                    j += 1
                    m += 1
                if not flag:
                    return False
        return True
