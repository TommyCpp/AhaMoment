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
    public TreeNode insertIntoBST(TreeNode root, int val) {
        if(root == null){
            return new TreeNode(val);
        }
        if(root.left == null && root.right == null){
            if(val > root.val){
                root.right = new TreeNode(val);
            }
            else{
                root.left = new TreeNode(val);
            }
            return root;
        }
        else{
            if(val > root.val){
                root.right = insertIntoBST(root.right, val);
            }
            else{
                root.left = insertIntoBST(root.left,val);
            }
            return root;
        }
    }
}
