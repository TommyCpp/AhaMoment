/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
class Solution {
    public ListNode reverseBetween(ListNode head, int m, int n) {
        ListNode before = null;
        ListNode p = head;
        for(int i = 0;i < m - 1;i++){
            before = p;
            p = p.next;
        }
        if(m == 1){
            ListNode dummy = new ListNode(-1); // if reverse starts at beginning
            reverse(dummy, p, n - m);
            return dummy.next;
        }
        reverse(before, p, n - m);
        
        return head;
    }
    
    public void reverse(ListNode before, ListNode p, int n){
        ListNode t = before;
        ListNode first = p;
        ListNode q = p.next;
        for(int i = 0; i < n + 1;i++){
            p.next = t;
            if(i != n){
                t = p;
                p = q;
                q = q.next;
            }
        }
        ListNode after = q;
        first.next = after;
        if(before != null){
            before.next = p;
        }
    }
}
