class UF:
  def __init__(self):
    self.data = [-1]*12
  def union(self, a,b):
    if self.data[a - 1] == self.data[b - 1]:
      self.data[a - 1] += self.data[b - 1]
      self.data[b - 1] = a
    elif self.data[a - 1] < 0 and self.data[b - 1] < 0 and self.data[a - 1] > self.data[b - 1]:
      self.data[b - 1] += self.data[a - 1]
      self.data[a - 1] = b
    s = "|"
    for i in self.data:
      s += str(i)+"   |"
    print(s)
