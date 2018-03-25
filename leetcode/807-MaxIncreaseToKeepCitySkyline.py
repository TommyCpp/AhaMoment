class Solution:
    def maxIncreaseKeepingSkyline(self, grid):
        """
        :type grid: List[List[int]]
        :rtype: int
        """
        """
        Max for every row and col
        Why don't you use something like enumerate?????
        """
        imax = [0 for i in range(len(grid))] 
        jmax = [0 for i in range(len(grid[0]))]
        
        for i in range(len(grid)):
            for j in range(len(grid[i])):
                value = grid[i][j]
                if value > imax[i]:
                    imax[i] = value
                if value > jmax[j]:
                    jmax[j] = value
        
        result = 0
        
        for i in range(len(grid)):
            for j in range(len(grid[j])):
                result += (min(imax[i],jmax[j]) - grid[i][j])
        
        return result
