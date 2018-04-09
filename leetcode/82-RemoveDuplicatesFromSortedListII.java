/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
class Solution {
    public ListNode deleteDuplicates(ListNode head) {
        if(head == null){
            return head;
        }
        else{
            ListNode p = head;
            if(head.next == null){
                return head;
            }
            while(head.next != null && head.val == head.next.val){
                head = head.next;
            }
            if(head.next == null){
                return null;
            }
            else{
                if(p == head){
                    head.next = this.deleteDuplicates(head.next);
                }
                else{
                    head = this.deleteDuplicates(head.next);
                }
                return head;
            }
        }
    }
}
