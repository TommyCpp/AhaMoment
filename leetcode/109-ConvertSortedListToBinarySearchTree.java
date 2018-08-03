/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
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
    public TreeNode sortedListToBST(ListNode head) {
        return this.bst(head, null);
    }
    
    public TreeNode bst(ListNode head, ListNode tail){
        if(head == null || head == tail) return null;
        if(head.next == tail){
            return new TreeNode(head.val);
        }
        ListNode p = head;
        ListNode jumper = head;
        while(jumper.next != null && jumper.next != tail && jumper.next.next != null && jumper.next.next != tail){
            jumper = jumper.next.next;
            p = p.next;
        }
        if(jumper.next != null && jumper.next != tail){
            jumper = jumper.next;
        }
        TreeNode node = new TreeNode(p.val);
        node.left = this.bst(head, p);
        node.right = this.bst(p.next, jumper.next);
        return node;
    }
}
