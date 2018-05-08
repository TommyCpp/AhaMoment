# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None
import collections
class Solution:
    def findFrequentTreeSum(self, root):
        """
        :type root: TreeNode
        :rtype: List[int]
        """
        sum = []
        sumTree(root,sum)
        counter = collections.Counter(sum)
        most_common = counter.most_common()
        if len(most_common) == 0:
            return []
        else:
            res = []
            for i,j in most_common:
                if j == most_common[0][1]:
                    res.append(i)
        return res
                    
                    
        
        
def sumTree(root, sum):
    if root == None:
        return 0
    else:
        value = root.val + sumTree(root.right,sum) + sumTree(root.left,sum)
        sum.append(value)
        return value
