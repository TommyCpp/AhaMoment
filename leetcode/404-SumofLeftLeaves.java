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
    public int sumOfLeftLeaves(TreeNode root) {
        if(root == null){
            return 0;
        }
        if(root.left == null){
            helper(root.right);
        }
        else{
            helper(root);
        }
        return this.res;
    }
    
    public void helper(TreeNode root){
        if(root == null){
            return ;
        }
        if(root.left != null){
            if(root.left.left == null){
                if(root.left.right == null){
                 this.res += root.left.val;
                }
                else{
                    helper(root.left.right);
                }
            }
            else{
                helper(root.left);
            }
        }
        if(root.right != null){
            helper(root.right);
        }
    }
}
