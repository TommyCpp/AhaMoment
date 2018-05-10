class Solution:
    def shortestCompletingWord(self, licensePlate, words):
        """
        :type licensePlate: str
        :type words: List[str]
        :rtype: str
        """
        chars = {}
        for s in licensePlate:
            if s.isalpha():
                s = s.lower()
                if s in chars.keys():
                    chars[s] += 1
                else:
                    chars[s] = 1

        words = sorted(words, key=lambda x:len(x),reverse=True)
        min_len = 16
        min_word = None
        for word in words:
            if len(word) < min_len and hasAlpha(word,chars):
                min_len = len(word)
                min_word = word
                        
        return min_word
                

def hasAlpha(word, chars):
    word = word.lower()
    for char, times in chars.items():
        index = word.find(char)
        if index == -1:
            return False
        else:
            res = 0
            while index != -1:
                res += 1
                index = word.find(char, index + 1)
            if res < times:
                return False
    return True
        
