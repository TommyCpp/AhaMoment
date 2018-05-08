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
    public TreeNode invertTree(TreeNode root) {
        if(root == null){
            return null;
        }
        else{
            TreeNode t = this.invertTree(root.left);
            TreeNode r = this.invertTree(root.right);
            root.left = r;
            root.right = t;
            return root;    
        }
    }
}
