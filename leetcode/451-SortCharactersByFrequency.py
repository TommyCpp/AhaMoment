import collections
class Solution:
    def frequencySort(self, s):
        """
        :type s: str
        :rtype: str
        """
        return "".join([s[0]*s[1]  for s in collections.Counter(s).most_common()])
