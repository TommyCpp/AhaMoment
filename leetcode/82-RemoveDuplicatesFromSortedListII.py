# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def deleteDuplicates(self, head):
        """
        :type head: ListNode
        :rtype: ListNode
        """
        if head == None:
            return head
        pre = ListNode(None)
        pre.next = head
        result = pre
        p = head
        nxt = p.next
        while(nxt != None):
            if p.val != nxt.val:
                pre = p
                p = p.next
                nxt = p.next
            else:
                while nxt != None and p.val == nxt.val:
                    p = p.next
                    nxt = nxt.next
                pre.next = nxt
                if nxt == None:
                    break
                p = pre.next
                nxt = p.next
        
        return result.next
