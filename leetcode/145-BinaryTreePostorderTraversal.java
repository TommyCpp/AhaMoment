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
    public List<Integer> postorderTraversal(TreeNode root) {
        Stack<TreeNode> stack = new Stack();
        Stack<Integer> res = new Stack();
        if(root == null){
            return res;
        }
        else{
            TreeNode p = root;
            stack.add(p);
            while(!stack.isEmpty()){
                p = stack.pop();
                res.add(p.val);
                if(p.left != null){
                    stack.add(p.left);
                }
                if(p.right != null){
                    stack.add(p.right);
                }
                
            }
        }
        List<Integer> lst = new ArrayList();
        while(!res.isEmpty()){
            lst.add(res.pop());
        }
        return lst;
    }
}
