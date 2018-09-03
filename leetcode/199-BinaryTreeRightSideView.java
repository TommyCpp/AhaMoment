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
    public List<Integer> rightSideView(TreeNode root) {
        if(root == null){
            return new ArrayList();
        }
        List<Integer> res = new ArrayList();
        LinkedList<TreeNode> cur = new LinkedList();
        LinkedList<TreeNode> last = new LinkedList();
        last.add(root);
        while(last.size() != 0){
            cur = new LinkedList();
            for(TreeNode node:last){
                if(node.left != null){
                cur.add(node.left);
                }
                if(node.right != null){
                    cur.add(node.right);
                }
            }
            res.add(last.pollLast().val);
            last = cur;
        }
        return res;
    }
}
