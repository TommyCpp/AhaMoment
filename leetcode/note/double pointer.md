## Double Pointer
Double Pointer can **get the middle of the LinkedList** because the `fast` move two node every time.

### Example question
https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/description/

* Use two pointer to find the duplicate. Given two pointer `slow` and `fast`.

* Initial them by set the `fast` as the next of `slow`

* While `fast` is not `Null`

* Iterator both `fast` and `slow` util `fast` value is equals to `slow` value (detect the duplicate)

* Then iterator only `fast` to the last node of duplicate.

* Then set `slow.next` to `fast.next`



### Example Code
```java
public class 
```Solution {
public ListNode deleteDuplicates(ListNode head) {
	//use two pointers, slow - track the node before the dup nodes, 
	// fast - to find the last node of dups.
    ListNode dummy = new ListNode(0), fast = head, slow = dummy;
    slow.next = fast;
    while(fast != null) {
    	while (fast.next != null && fast.val == fast.next.val) {
     		fast = fast.next;    //while loop to find the last node of the dups.
    	}
    	if (slow.next != fast) { //duplicates detected.
    		slow.next = fast.next; //remove the dups.
    		fast = slow.next;     //reposition the fast pointer.
    	} else { //no dup, move down both pointer.
    		slow = slow.next;
    		fast = fast.next;
    	}
    	
    }
    return dummy.next;
} }
```
