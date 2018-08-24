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
    public int kthSmallest(TreeNode root, int k) {
        return this.getSeq(root).get(k - 1).val;
    }
    
    public List<TreeNode> getSeq(TreeNode root){
        if(root == null){
            return new ArrayList();
        }
        else{
            List res = this.getSeq(root.left);
            res.add(root);
            res.addAll(this.getSeq(root.right));
            return res;
        }
    }
}
