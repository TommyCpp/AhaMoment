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
    public TreeNode pruneTree(TreeNode root) {
        if(root == null){
            return null;
        }
        if(root.left != null || root.right != null){
            root.left = this.pruneTree(root.left);
            root.right = this.pruneTree(root.right);
        }
        if(root.val == 0 && root.left == null && root.right == null){
            return null;
        }
        else{
            return root;
        }
        
    }
}
