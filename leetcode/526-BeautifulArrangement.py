class Solution:
    def countArrangement(self, N):
        """
        :type N: int
        :rtype: int
        """
        def backtracking(length, nums):
            res = []
            if len(nums) == 1:  # In the end
                if nums[0] % length == 0 or length % nums[0] == 0:
                    return [[nums[0]]]
                else:
                    return []
            else:
                for num in nums:
                    if num % length == 0 or length % num == 0:
                        for lst in backtracking(length + 1,[j for j in nums if j != num]):
                            lst.insert(0, num)
                            res.append(lst)
                return res
        return len(backtracking(1, range(1, N + 1)))
            
        
