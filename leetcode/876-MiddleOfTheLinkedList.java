/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
class Solution {
    public ListNode middleNode(ListNode head) {
        if(head == null) return head;
        ListNode walker = head;
        ListNode runner = head;
        while(runner != null && runner.next != null && runner.next.next != null){
            walker = walker.next;
            runner = runner.next.next;
        }
        if(runner.next != null){
            return walker.next;
        }
        else{
            return walker;
        }
        
    }
}
