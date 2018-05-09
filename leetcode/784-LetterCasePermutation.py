class Solution:
    def letterCasePermutation(self, S):
        """
        :type S: str
        :rtype: List[str]
        """
        def backtracking(S):
            res = []
            if len(S) == 0:
                return [""]
            if len(S) == 1: # End situation
                if S.isnumeric():
                    return [S]
                else:
                    return [S, S.lower() if S.isupper() else S.upper()]
            else:
                if S[0].isnumeric():
                    for s in backtracking(S[1:]):
                        res.append(S[0] + s)
                else:
                    for s in backtracking(S[1:]):
                        res.append(S[0] + s)
                        res.append(str(S[0].lower() if S[0].isupper() else S[0].upper()) + s)
                return res
        return backtracking(S) 
