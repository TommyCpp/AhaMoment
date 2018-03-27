class Solution:
    def constructMaximumBinaryTree(self, nums):
        """
        :type nums: List[int]
        :rtype: TreeNode
        
        Iteration
        Time complexity: O(n^2)
        """
        if len(nums) == 0:
            return None
        maxValue = max(nums)
        maxIndex = nums.index(maxValue)
        root = TreeNode(maxValue)
        root.left = self.constructMaximumBinaryTree(nums[0:maxIndex])
        root.right = self.constructMaximumBinaryTree(nums[maxIndex+1:]) if maxIndex < len(nums) else None
        return root
