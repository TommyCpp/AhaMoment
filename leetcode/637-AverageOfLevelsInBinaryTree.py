# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def averageOfLevels(self, root):
        """
        :type root: TreeNode
        :rtype: List[float]
        """
        layer = []
        def layerCounter(root,i):
            if root.right or root.left:
                if len(layer) < i + 1:
                    layer.append([])
                if root.left:
                    layer[i].append(root.left.val)
                    layerCounter(root.left, i + 1)
                if root.right:
                    layer[i].append(root.right.val)
                    layerCounter(root.right, i + 1)
            
        layer.append([root.val])
        layerCounter(root,1)
        res = [0]*len(layer)
        for i, lst in enumerate(layer):
            res[i] = sum(lst) / len(lst)
        
        return res
