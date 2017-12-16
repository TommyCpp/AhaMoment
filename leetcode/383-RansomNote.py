class Solution:
    def canConstruct(self, ransomNote, magazine):
        """
        :type ransomNote: str
        :type magazine: str
        :rtype: bool
        """
        ranDic = collections.Counter(ransomNote)
        magDic = collections.Counter(magazine)
        for k in ranDic.keys():
            if k in magDic.keys():
                if ranDic[k] > magDic[k]:
                    return False
            else:
                return False
        return True
