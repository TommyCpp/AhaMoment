/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
class Solution {
    public ListNode rotateRight(ListNode head, int k) {
        if(head == null || k == 0 || head.next == null){
            return head; // 如果是空链表或者只有一个节点的链表或者k为0，则返回原链表
        }
        int count = 0;
        ListNode pre = new ListNode(-1);
        pre.next = head;
        ListNode p = head;
        while(p != null){
            count ++;
            p = p.next;
        }
        k = count - k % count; //注意此时k表示第k个节点为最后一个节点
        if(k == count){
            return head; //如果k == count，表明原链表的最后一个节点在结果链表中仍然是最后一个节点，此时返回原链表即可
        }
        p = pre;
        int i = 0;
        while(p != null && i < k){
            p = p.next;
            i ++ ;
        }
        ListNode q = p.next;
        pre.next = q;
        ListNode t = q;
        p.next = null;
        while(t.next != null){
            t = t.next;
        }
        t.next = head;
        return pre.next;
    }
}
