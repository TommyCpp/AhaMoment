/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) {
 *         val = x;
 *         next = null;
 *     }
 * }
 */
public class Solution {
    public ListNode getIntersectionNode(ListNode headA, ListNode headB) {
        int lengthA = 0;
        int lengthB = 0;
        ListNode a = headA;
        ListNode b = headB;
        while(a != null){
            a = a.next;
            lengthA ++;
        }
        while(b != null){
            b = b.next;
            lengthB ++;
        }
        a = headA;
        b = headB;
        System.out.println(lengthA);
        
        if(lengthA >= lengthB){
            while(lengthA > lengthB){
                a = a.next;
                lengthA --;
            }
        }
        else{
            while(lengthB > lengthA){
                b = b.next;
                lengthB --;
            }
        }
        while(a != null && b != null && a != b){
            a = a.next;
            b = b.next;
        }
        return a;
    }
}
