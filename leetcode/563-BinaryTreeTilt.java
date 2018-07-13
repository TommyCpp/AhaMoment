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
    public int findTilt(TreeNode root) {
        root = this.sumTree(root);
        this.tile(root);
        return this.res;
    }
    
    public void tile(TreeNode root){
        if(root != null){
            int leftVal = root.left == null ? 0:root.left.val;
            int rightVal = root.right == null? 0:root.right.val;
            this.res += Math.abs(leftVal - rightVal);
            tile(root.right);
            tile(root.left);
        }
    }
    
    public TreeNode sumTree(TreeNode root){
        if(root == null){
            return null;
        }
        else{
            root.left = sumTree(root.left);
            root.right = sumTree(root.right);
            if(root.left != null){
                root.val += root.left.val;
            }
            if(root.right != null){
                root.val += root.right.val;
            }
            return root;
        }
    }
}
