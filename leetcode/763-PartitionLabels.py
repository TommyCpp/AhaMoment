class Solution:
    def partitionLabels(self, S):
        """
        :type S: str
        :rtype: List[int]
        """
        start = [-1]*26
        end = [-1]*26
        for i,s in enumerate(S):
            if start[ord(s) - 97] == -1:
                start[ord(s) - 97] = i
            end[ord(s) - 97] = i
        ai = list(zip(start,end)) # stands for alpha_index
        ai.sort(key=lambda x:x[0]) # sort by start index
        ai = list(filter(lambda x:x[0] != -1 and x[1] != -1,ai))
        result = []
        cs =  -1 # stands for current_start
        ce =  -1 # stands for current_end
        for i in range(len(ai)):
            if cs == -1 and cs == -1:
                cs = ai[i][0]
                ce = ai[i][1]
            if ai[i][0] > ce:
                result.append((ce-cs) + 1)
                cs = ai[i][0]
                ce = ai[i][1]
            if ai[i][0] > cs and ai[i][1] > ce:
                ce = ai[i][1]
        result.append((ce-cs)+1) # add the last one
        return result
