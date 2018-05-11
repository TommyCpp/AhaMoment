class Solution:
    def findDuplicate(self, paths):
        """
        :type paths: List[str]
        :rtype: List[List[str]]
        """
        table = {}
        for path in paths:
            file_path, files = processPath(path)
            for file in files:
                if file[0] in table.keys():
                    table[file[0]].append("{}/{}".format(file_path,file[1]))
                else:
                    table[file[0]] = ["{}/{}".format(file_path,file[1])]
                    
        return list(filter(lambda x:len(x) > 1, list(table.values())))

def processPath(path):
    file_path = None
    files = [] # list[(content, filename)]
    for index, value in enumerate(path.split(" ")):
        if index == 0:
            file_path = value
        else:
            filename, content = value.split("(")
            files.append((content[:-1], filename))
    return file_path, files
        
