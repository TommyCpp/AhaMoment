class Solution:
    def findTarget(self, root, k):
        """
        :type root: TreeNode
        :type k: int
        :rtype: bool
        """
        self.values = set()
        if root == None:
            return False
        if root.left == root.right == None:
            return False
        return self.iterator(root, k)

    def iterator(self, root, k):
        if root == None:
            return False
        else:
            if k -  root.val in self.values:
                return True
            else:
                self.values.add(root.val)
                return self.iterator(root.left, k) or self.iterator(root.right, k)
