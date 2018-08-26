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
    public boolean leafSimilar(TreeNode root1, TreeNode root2) {
        List seq1 =  new ArrayList();
        this.getLeafSeq(root1,seq1);
        List seq2 = new ArrayList();
        this.getLeafSeq(root2, seq2);
        if(seq1.size() != seq2.size()){
            return false;
        }
        else{
            for(int i = 0; i < seq1.size();i++){
                if(seq1.get(i) != seq2.get(i)){
                    return false;
                }
            }
            return true;
        }
    }
    
    public void getLeafSeq(TreeNode node, List<Integer> seq){
        if(node == null){
            return ;
        }
        if(node.left == null && node.right == null){
            seq.add(node.val);
            return ;
        }
        else{
            getLeafSeq(node.left ,seq);
            getLeafSeq(node.right, seq);
        }
    }
}
