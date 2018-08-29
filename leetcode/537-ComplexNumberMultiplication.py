class Solution:
    def complexNumberMultiply(self, a, b):
        """
        :type a: str
        :type b: str
        :rtype: str
        """
        real_a = (int)(a.split("+")[0])
        real_b = (int)(b.split("+")[0])
        imaginary_a = (int)(a.split("+")[1].replace("i",""))
        imaginary_b = (int)(b.split("+")[1].replace("i",""))
        return "{}+{}i".format(real_a * real_b - imaginary_a * imaginary_b , imaginary_a * real_b  + imaginary_b * real_a)
