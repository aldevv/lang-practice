a = iter([1, 2, 3, 4, 5, 6])


def show_nums():
    for i in range(10):
        yield from a


print(next(show_nums()))
print(next(show_nums()))
print(next(show_nums()))
print(next(show_nums()))
print(next(show_nums()))
