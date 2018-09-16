/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) { val = x; }
 * }
 */
class Solution {
    public ListNode mergeKLists(ListNode[] lists) {
        if(lists.length == 0){
            return null;
        }
        if(lists.length == 1){
            return lists[0];
        }
        else{
            if(lists.length == 2){
                ListNode p = lists[0];
                ListNode q = lists[1];
                ListNode res = new ListNode(-1);
                ListNode t = res;
                
                while(p != null && q != null){
                    if(p.val < q.val){
                        t.next = p;
                        p = p.next;
                    }
                    else{
                        t.next = q;
                        q = q.next;
                    }
                    t = t.next;
                }
                
                while(p!= null){
                    t.next = p;
                    p = p.next;
                    t = t.next;
                }
                
                while(q!=null){
                    t.next = q;
                    q = q.next;
                    t = t.next;
                }
                return res.next;
            }
            else{
                int median = lists.length / 2;
                ListNode[] firstPart = Arrays.copyOfRange(lists, 0,median);
                ListNode[] lastPart = Arrays.copyOfRange(lists, median, lists.length);
                ListNode a = mergeKLists((ListNode[])firstPart);
                ListNode b = mergeKLists((ListNode[])lastPart);
                ListNode[] next ={a, b};
                return mergeKLists(next);
            }
        }
    }
}
