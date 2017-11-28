class Solution:
    def judgeCircle(self, moves):
        """
        :type moves: str
        :rtype: bool
        """
        num_U = len(re.findall("U",moves))
        num_D = len(re.findall("D",moves))
        num_L = len(re.findall("L",moves))
        num_R = len(re.findall("R",moves))
        return num_U == num_D and num_L == num_R
