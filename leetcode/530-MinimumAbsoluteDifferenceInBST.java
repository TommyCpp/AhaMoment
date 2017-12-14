/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode(int x) { val = x; }
 * }
 */
 /*
  * 比较两个搜索树（当前值-左子树最大值，右子树最小值-当前值， 左子树中的最小差值，右子树中最小差值）中最小的
  */
class Solution {
    public int getMinimumDifference(TreeNode root) {
        int leftV,rightV;
        if(root.left == null){
            leftV = Integer.MAX_VALUE;
        }
        else{
            leftV = Math.min(leftMin(root),getMinimumDifference(root.left));
        }
        if(root.right == null){
            rightV = Integer.MAX_VALUE;
        }
        else{
            rightV = Math.min(rightMin(root),getMinimumDifference(root.right));
        }
        return Math.min(leftV,rightV);
    }
    
    public int rightMin(TreeNode root){
        int a = root.val;
        TreeNode r = root.right;
        if(r == null){
            return Integer.MAX_VALUE;
        }
        while(r.left != null){
            r = r.left;
        }
        return r.val - a;
    }
    
    public int leftMin(TreeNode root){
        int a = root.val;
        TreeNode l = root.left;
        if(l == null){
            return Integer.MAX_VALUE;
        }
        while(l.right != null){
            l = l.right;
        }
        return a - l.val;
    }
}
