class DBHandler:
    processed = []

    def get_data(self):
        return [f"item {i}, (row: {i})" for i in range(10)]


class Coroutine:
    def run(self):
        s = DBHandler()
        default = [None] * 10
        while True:
            try:
                processed_data = (yield s.get_data()) or default
                s.__class__.processed += processed_data
            except GeneratorExit:
                print("exit plz")
                break


def main():
    c = Coroutine()
    r = c.run()

    print("before:")
    print(DBHandler.processed)

    # first time only to get to the yield
    values = next(r)

    # r.send goes until next yield so no need for next() again
    values = r.send([v.split(",")[0].split()[1] for v in values])

    print("after:")
    print(DBHandler.processed)

    values = r.send([v.split(",")[0].split()[1] for v in values])

    print("after: 2")
    print(DBHandler.processed)

    r.close()


if __name__ == "__main__":
    main()
