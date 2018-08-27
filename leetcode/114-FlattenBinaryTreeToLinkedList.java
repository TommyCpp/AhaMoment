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
    public void flatten(TreeNode root) {
        if(root == null){
            return ;
        }
        else {
            TreeNode left = root.left;
            TreeNode right = root.right;
            this.flatten(left);
            root.right = left;
            root.left = null;
            TreeNode p = root;
            while(p.right != null){
                p = p.right;
            }
            this.flatten(right);
            p.right = right;
        }
    }
}
