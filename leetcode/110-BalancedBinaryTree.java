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
    public boolean isBalanced(TreeNode root) {
        if(root == null){
            return true;
        }
        else{
            int res = depth(root);
            return res > 0;
        }
    }
    
    public int depth(TreeNode root){
        if(root == null){
            return 0;
        }
        else{
            int left = this.depth(root.left);
            int right = this.depth(root.right);
            if(left < 0 || right < 0){
                return -1;
            }
            if(Math.abs(right - left) <= 1){
                return left > right ? left + 1: right + 1;
            }
            else{
                return -1;
            }
        }
    }
}
