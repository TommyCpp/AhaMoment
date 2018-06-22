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
    public int sum = 0;
    public TreeNode convertBST(TreeNode root) {
        convert(root);
        return root;
    }
    public void convert(TreeNode root){
        if(root == null){
            return;
        }
        else{
            this.convert(root.right);
            sum += root.val;
            root.val = sum;
            this.convert(root.left);
        }
    }
    
}
