# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def reverseList(self, head):
        """
        :type head: ListNode
        :rtype: ListNode
        """
        if head == None:
            return None
        if head.next == None:
            return head
        else:
            p_1 = None
            p_2 = head
            p_3 = head.next
            
            while 1:
                p_2.next = p_1
                
                if p_3 == None:
                    break
                
                p_1 = p_2
                p_2 = p_3
                p_3 = p_3.next
        
            return p_2
