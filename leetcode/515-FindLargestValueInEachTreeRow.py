# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None
import sys
class Solution:
    def largestValues(self, root):
        """
        :type root: TreeNode
        :rtype: List[int]
        """
        preRow = []
        row = [root]
        res = []
        if root == None:
            return []
        while len(row) != 0: 
            preRow = []
            max = -sys.maxsize -1
            for node in row:
                if node.val > max:
                    max = node.val
                preRow.append(node)
            res.append(max)
            row = []
            for node in preRow:
                if node.left:
                    row.append(node.left)
                if node.right:
                    row.append(node.right)
        return res
