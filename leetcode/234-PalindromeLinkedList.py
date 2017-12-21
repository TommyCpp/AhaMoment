class Solution:
    def isPalindrome(self, head):
        """
        :type head: ListNode
        :rtype: bool
        """
        stack = []
        p = head
        while p != None:
            stack.append(p.val)
            p = p.next
        q = head
        while q != None:
            if q.val != stack.pop():
                return False
            q = q.next
        return True
