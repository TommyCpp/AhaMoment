class Solution:
    def minCostClimbingStairs(self, cost):
        """
        :type cost: List[int]
        :rtype: int
        """
        cost.append(0)  # Add the destination
        target  = len(cost)
        d = [0]*(target+1) # Add start
        for i in range(1,target+1):
            if i == 1:
                d[1] = cost[0]
            else:
                # print(i,d[i-1],d[i-2],cost[i-1])
                d[i] = min(d[i-1]+cost[i-1],d[i-2]+cost[i-1])
        
        return d[target]
