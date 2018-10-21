/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode(int x) { val = x; }
 * }
 */
class Solution {
    int res = 0;
    public int longestConsecutive(TreeNode root) {
        if(root == null){
            return 0;
        }
        helper(root, root.val - 1, 0); // note that we use 0 and root.val - 1 to start
        return res;
    }
    
    public void helper(TreeNode root, int parent, int cur){
        if(root == null){
            return;
        } else{
            if(root.val == parent + 1){
                res = cur + 1 > res ? cur + 1: res;
                helper(root.left, root.val, cur + 1);
                helper(root.right, root.val, cur + 1);
            } else{
                helper(root.left, root.val, 1); // but in tree we use 1 to start
                helper(root.right, root.val, 1);
            }
        }
    }
}
