class Solution:
    def intersection(self, nums1, nums2):
        """
        :type nums1: List[int]
        :type nums2: List[int]
        :rtype: List[int]
        """
        set1 = {}
        set2 = {}
        for i in nums1:
            set1[i] = 0
        for i in nums2:
            set2[i] = 0
        res = []
        for i in set1.keys():
            if i in set2.keys():
                res.append(i)
        return res
