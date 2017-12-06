class Solution:
    def canPlaceFlowers(self, flowerbed, n):
        """
        :type flowerbed: List[int]
        :type n: int
        :rtype: bool
        """
        if n == 0:
            return True
        if len(flowerbed) == 1:
            return flowerbed[0] == 0
        if len(flowerbed) == 2:
            if n >= 2:
                return False
            else:
                return flowerbed[0] == flowerbed[1] and flowerbed[0] == 0
        flowerbed.append(flowerbed[-2])
        flowerbed.insert(0,flowerbed[1])
        count = 0
        for i in range(1,len(flowerbed)-1):
            if not (flowerbed[i] == 1 or flowerbed[i-1] == 1 or  flowerbed[i+1] == 1):
                count += 1
                flowerbed[i] = 1
                if count >= n:
                    return True
        return count >= n
