# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def numComponents(self, head, G):
        """
        :type head: ListNode
        :type G: List[int]
        :rtype: int
        """
        G = set(G)
        res = 0
        p = head
        flag = False
        while p:
            if p.val in G:
                p = p.next
                flag = True
            else:
                if flag:
                    flag = False
                    res += 1
                    while p and p.val not in G:
                        p = p.next
                else:
                    p = p.next
        if flag:
            res += 1
        return res
