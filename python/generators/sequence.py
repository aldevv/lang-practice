from itertools import tee

seq = iter([9, 8, 7, 6, 5, 4, 3, 2, 1])

first, second = tee(seq, 2)
print(min(first))
print(max(second))
