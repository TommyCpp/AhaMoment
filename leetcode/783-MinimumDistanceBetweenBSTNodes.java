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
    public int minDiffInBST(TreeNode root) {
        if(root == null){
            return Integer.MAX_VALUE;
        }
        else{
            int leftVal = Integer.MAX_VALUE, rightVal = Integer.MAX_VALUE;
            TreeNode left = findRight(root.left);
            TreeNode right = findLeft(root.right);
            if(left != null){
                leftVal = Math.abs(left.val - root.val);
            }
            if(right != null){
                rightVal = Math.abs(right.val - root.val);
            }
            int a = Integer.min(leftVal, rightVal);
            int b = Integer.min(minDiffInBST(root.left),minDiffInBST(root.right));
            return Integer.min(a,b);
        }
        
        
    }
    
    public TreeNode findLeft(TreeNode root){
        if(root == null){
            return null;
        }
        while(root.left != null){
            root = root.left;
        }
        return root;
    }
    
    public TreeNode findRight(TreeNode root){
        if(root == null){
            return null;
        }
        while(root.right != null){
            root = root.right;
        }
        return root;
    }
}
