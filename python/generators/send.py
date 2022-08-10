# send, differentiates generators from coroutines


def show_nums():
    yield 0
    rcvd = yield 10
    yield "test: " + rcvd
    yield 30


s = show_nums()
print(next(s))
print(next(s))

try:
    print(s.send("sent"))
    print(next(s))
except StopIteration:
    print("failed")
