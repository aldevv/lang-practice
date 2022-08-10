class DBHandler:
    def get_data(self):
        return [f"item {i}, (row: {i})" for i in range(10)]


class Coroutine:
    def run(self):
        s = DBHandler()
        while True:
            try:
                yield s.get_data()
            except CustomException:
                print("my custom exception was executed")
            except GeneratorExit:
                print("exit plz")
                break


class CustomException(Exception):
    pass


def main():
    c = Coroutine()
    r = c.run()
    print(next(r))
    print(next(r))
    r.throw(CustomException)
    print(next(r))
    # this also executes when the generator is picked up by the
    # garbage collector, so it will execute even without the close()
    r.close()


if __name__ == "__main__":
    main()
