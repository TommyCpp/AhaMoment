import collections
class Solution:
    def mostCommonWord(self, paragraph, banned):
        """
        :type paragraph: str
        :type banned: List[str]
        :rtype: str
        """
        paragraph = paragraph.lower()
        punctuation = ["!","?","'",",",";","."]
        alphaParagraph = []
        word = ""
        for char in paragraph:
            if char == " ":
                alphaParagraph.append(word)
                word = ""
            elif char not in punctuation:
                word += char
        alphaParagraph.append(word)

        counter = collections.Counter(alphaParagraph)
        for word, _ in counter.most_common():
            if word not in banned:
                return word
