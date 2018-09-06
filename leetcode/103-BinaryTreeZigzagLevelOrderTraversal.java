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
    public List<List<Integer>> zigzagLevelOrder(TreeNode root) {
        if(root == null){
            return new ArrayList<List<Integer>>();
        }
        //boolean isStack = false;
        List<List<Integer>> result = new ArrayList();
        Stack<TreeNode> last = new Stack();
        Stack<TreeNode> cur = new Stack();
        last.add(root);
        while(!last.isEmpty() || !cur.isEmpty()){
            List<Integer> level = new ArrayList();
            if(!last.isEmpty()){
                while(!last.isEmpty()){
                    TreeNode node = last.pop();
                    level.add(node.val);
                    if(node.left != null){
                        cur.push(node.left);
                    }
                    if(node.right != null){
                        cur.push(node.right);
                    }
                }
            }
            else{
                while(!cur.isEmpty()){
                    TreeNode node = cur.pop();
                    level.add(node.val);
                    if(node.right != null){
                        last.push(node.right);
                    }
                    if(node.left != null){
                        last.push(node.left);
                    }
                }
            }
            result.add(level);
        }
        return result;
        
    }
}
