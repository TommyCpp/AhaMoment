class Solution:
    def reconstructQueue(self, people):
        """
        :type people: List[List[int]]
        :rtype: List[List[int]]
        """
        res = []
        people = sorted(people, key=lambda x: x[1]) # make sure that sort by x[1] when x[0] is equal
        people = sorted(people, key=lambda x:-x[0])
        for i, j in people:
            res.insert(j,[i,j])
        return res
