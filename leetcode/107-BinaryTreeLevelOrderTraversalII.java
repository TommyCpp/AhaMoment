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
    public List<List<Integer>> levelOrderBottom(TreeNode root) {
        if(root == null){
            return new ArrayList();
        }
        Stack<List<Integer>> stack = new Stack();
        Queue<TreeNode> old = new ArrayDeque();
        Queue<TreeNode> next = new ArrayDeque();
        old.add(root);
        while(!old.isEmpty() || !next.isEmpty()){
            List<Integer> levelRes = new ArrayList();
            while(!old.isEmpty()){
                TreeNode node = old.poll();
                levelRes.add(node.val);
                if(node.left != null){
                    next.add(node.left);
                }
                if(node.right != null){
                    next.add(node.right);
                }
            }
            old = next;
            next = new ArrayDeque();
            stack.add(levelRes);
        }
        List<List<Integer>> res = new ArrayList();
        while(!stack.isEmpty()){
            res.add(stack.pop());   
        }
        return res;
    }
}
