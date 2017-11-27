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
    public TreeNode addOneRow(TreeNode root, int v, int d) {
        TreeNode result = null;
         if(d == 1){
             result = new TreeNode(v);
             result.left =  root;
             result.right = null;
         }
        else{
            result = addOneRow1(root,v,d-1);
        }
        return result;
    }
    
    public TreeNode addOneRow1(TreeNode root,int v,int d){
        if(root == null){
             return null;
         }
         if(d == 1){
             TreeNode temp_l = root.left;
             TreeNode temp_r = root.right;
             TreeNode l = new TreeNode(v);
             TreeNode r = new TreeNode(v);
             l.left = temp_l;
             r.right = temp_r;
             root.left = l;
             root.right = r;
         }
        else{
            root.left = addOneRow1(root.left,v,d-1);
            root.right = addOneRow1(root.right,v,d-1);
        }
        return root;
    }
}
