def fib():

    yield 0
    yield 1
    prev = 0
    cur = 1
    while True:
        c = cur + prev
        cur, prev = c, cur
        yield cur


f = fib()
print(next(f))
print(next(f))
print(next(f))
print(next(f))
print(next(f))
print(next(f))
print(next(f))
print(next(f))
print(next(f))
print(next(f))
# print(f)
# for i in f:
#     print(i)
