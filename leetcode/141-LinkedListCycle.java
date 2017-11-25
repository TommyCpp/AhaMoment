public class Solution {
    public boolean hasCycle(ListNode head) {
            Set<ListNode> nodes = new HashSet<>();
            ListNode p = head;
            while(p!=null){
                if(nodes.contains(p)){
                    return true;
                }
                nodes.add(p);
                p = p.next;
            }
            return false;
    }
}
