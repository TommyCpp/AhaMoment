/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
class Solution {
    public ListNode removeElements(ListNode head, int val) {
        if(head == null){
            return null;
        }
        if(head.next == null){
            if(head.val == val){
                return null;
            }
            else{
                return head;
            }
        }
        ListNode pre = new ListNode(0);
        pre.next = head;
        ListNode dummy = pre;
        ListNode cur = head;
        ListNode next = head.next;
        while(next != null){
            if(cur.val == val){
                pre.next = next;
                cur = next;
                next = next.next;
            }
            else{
                pre = cur;
                cur = next;
                next = next.next;
            }
        }
        if(cur.val == val){
            pre.next = null;
        } //if tha last element is val
        return dummy.next;
    }
}
