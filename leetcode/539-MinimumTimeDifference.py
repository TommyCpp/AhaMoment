class Solution:
    def findMinDifference(self, timePoints):
        """
        :type timePoints: List[str]
        :rtype: int
        """
        times = []
        for time in timePoints:
            h,m = time.split(":")
            times.append(int(h) * 60 + int(m))
        times.sort()
        times.append(times[0] + 1440) # add the first one plus 1440 to the end of times
        res = 1440
        for i in range(len(times) - 1):
            res = min(res, times[i + 1] - times[i])
        return res
