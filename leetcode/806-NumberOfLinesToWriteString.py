class Solution:
    def numberOfLines(self, widths, S):
        """
        :type widths: List[int]
        :type S: str
        :rtype: List[int]
        """
        surplus = 0
        current = 0
        line = 0
        for s in S:
            width = widths[ord(s) - ord('a')]
            if current + width > 100:
                line += 1
                current = width
            else:
                current += width
                
        return [line + 1, current]
