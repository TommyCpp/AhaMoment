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
    public boolean isSubtree(TreeNode s, TreeNode t) {
        if(s == null && t == null){
            return true;
        }
        if(s == null && t != null || t == null && s != null){
            return false;
        }
        if(s.val == t.val){
            return sameTree(s,t) || isSubtree(s.left, t) || isSubtree(s.right, t);
        }
        else{
            return isSubtree(s.left, t) || isSubtree(s.right, t);
        }
    }
    
    public boolean sameTree(TreeNode s, TreeNode t){
        if(s == null && t == null){
            return true;
        }
        if(s == null && t != null || t == null && s != null){
            return false;
        }
        else{
            if(s.val == t.val){
                return sameTree(s.left, t.left) && sameTree(s.right, t.right);
            }
            else{
                return false;
            }
        }
    }
}
