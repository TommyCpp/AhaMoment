class Solution:
    def detectCapitalUse(self, word):
        """
        :type word: str
        :rtype: bool
        """
        return word == "" or (word != "" and (word.isupper() or word.islower() or ( word[0].isupper() and word[1:].islower() )))
