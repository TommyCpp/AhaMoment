# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None
import copy
class Solution:
    def findBottomLeftValue(self, root):
        """
        :type root: TreeNode
        :rtype: int
        """
        previousLevel = []
        level = [root]
        while len(level) != 0:
            previousLevel = []
            for node in level:
                previousLevel.append(node)
            level = []
            for node in previousLevel:
                if node.left:
                    level.append(node.left)
                if node.right:
                    level.append(node.right)
        return previousLevel[0].val
