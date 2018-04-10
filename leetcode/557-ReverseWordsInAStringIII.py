class Solution:
    def reverseWords(self, s):
        """
        :type s: str
        :rtype: str
        """
        return " ".join([sub[::-1] for sub in s.split(" ")])  # NOTE: use slice op to reverse string in python
