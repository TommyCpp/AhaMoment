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
    public List<List<Integer>> pathSum(TreeNode root, int sum) {
        if(root == null){
            return new LinkedList();
        }
        else{
            if(root.left == null && root.right == null && root.val == sum){
                List res = new LinkedList(); // Use LinkedList to fast the process to insert at front.
                List element = new LinkedList();
                element.add(root.val);
                res.add(element);
                return res;
            }else{
                List<List<Integer>> left = pathSum(root.left, sum - root.val);
                List<List<Integer>> right = pathSum(root.right, sum - root.val);
                for(List ele: left){
                    ele.add(0, root.val);
                }
                for(List ele: right){
                    ele.add(0, root.val);
                }
                left.addAll(right);
                return left;
            }
        }
    }
}
