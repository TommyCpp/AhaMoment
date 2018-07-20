/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
class Solution {
    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        ListNode res = new ListNode(0);
        ListNode p  = l1;
        ListNode q  = l2;
        ListNode t = res;
        int next = 0;
        while(p != null && q != null){
            int value = p.val + q.val + next;
            if(value < 10){
                t.next = new ListNode(value);
                next = 0;
            }
            else{
                t.next = new ListNode(value % 10);
                next = value / 10;
            }
            p = p.next;
            q = q.next;
            t = t.next;
        }
        while(p != null){
            int value = p.val + next;
            if(value < 10){
                t.next = new ListNode(value);
                next = 0;
            }
            else{
                t.next = new ListNode(value % 10);
                next = value / 10;
            }
            p = p.next;
            t = t.next;
        }
        while(q != null){
            int value = q.val + next;
            if(value < 10){
                t.next = new ListNode(value);
                next = 0;
            }
            else{
                t.next = new ListNode(value % 10);
                next = value / 10;
            }
            q = q.next;
            t = t.next;
        }
        if(next != 0){
            t.next = new ListNode(next);
        }
        return res.next;
    }
}
