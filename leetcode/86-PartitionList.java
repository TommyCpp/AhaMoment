/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
class Solution {
    public ListNode partition(ListNode head, int x) {
        ListNode before = new ListNode(-1);
        ListNode beforeIter = before;
        ListNode after = new ListNode(-1);
        ListNode afterIter = after;
        ListNode iterator = head;
        ListNode nextNode = null;
        
        while(iterator != null){
            if(iterator.val >= x){
                afterIter.next = iterator;
                afterIter = afterIter.next;
            }
            else{
                beforeIter.next = iterator;
                beforeIter = beforeIter.next;
            }
    
            nextNode = iterator.next;
            iterator.next = null;
            iterator = nextNode;
        }
        
        beforeIter.next = after.next;
        
        return before.next;
    }
}
