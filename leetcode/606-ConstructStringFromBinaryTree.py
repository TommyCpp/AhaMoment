class Solution:
    def tree2str(self, t):
        """
        :type t: TreeNode
        :rtype: str
        """
        return toStr(t)

def toStr(t):
    if t is None:
        return ""
    else:
        if t.left is None and t.right is None:
            return str(t.val)
        if t.left is None and t.right is not None:
            return "{}()({})".format(t.val,toStr(t.right))
        if t.left is not None and t.right is None:
            return "{}({})".format(t.val, toStr(t.left))
        else:
            return "{}({})({})".format(t.val, toStr(t.left), toStr(t.right))
