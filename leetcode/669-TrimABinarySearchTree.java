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
    public TreeNode trimBST(TreeNode root, int L, int R) {
        if(root == null){
            return null;
        }
        if(root.val == L){
            root.right = this.trimBST(root.right,L,R);
            root.left = null;
            return root;
        }
        if(root.val == R){
            root.left = this.trimBST(root.left,L,R);
            root.right = null;
            return root;
        }
        if(root.val < L){
            return this.trimBST(root.right,L,R);
        }
        if(root.val > R){
            return this.trimBST(root.left,L,R);
        }
        if(root.val > L && root.val < R){
            root.left = this.trimBST(root.left,L,R);
            root.right = this.trimBST(root.right,L,R);
            return root;
        }
        return null;
    }
}
