import functools


def type_check(correct_type):
    def decorator(func):
        @functools.wraps(func)
        def wrapper(arg):
            if isinstance(arg, correct_type):
                return func(arg)
            else:
                print("Bad Type")

        return wrapper

    return decorator


@type_check(int)
def times2(num):
    return num * 2


print(times2("hey"))
# times2("Not A Number")


@type_check(str)
def first_letter(word):
    return word[0]


print(first_letter("Hello World"))
# first_letter(["Not", "A", "String"])
