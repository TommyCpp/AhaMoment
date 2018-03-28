class Solution:
    """
    Note that for a acyclic graph, it means that every member of grpah[i] are guarantee to be bigger than i.
    """
    def findPath(self,graph,offset):
        target = offset + len(graph) - 1  # offset maintain the index of the original graph
        results = []
        if target in graph[0]:
            results.append([offset,target])
        
        for i in graph[0]:
            if i != target:
                for result in self.findPath(graph[i-offset:], i):
                    if len(result) != 0:
                        result.insert(0,offset)
                        results.append(result)
                        
        return results
        
        
        
        
    def allPathsSourceTarget(self, graph):
        """
        :type graph: List[List[int]]
        :rtype: List[List[int]]
        """
        return self.findPath(graph,0)
