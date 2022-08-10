import logging

logging.basicConfig(level=logging.INFO, format="%(message)s")
logger = logging.getLogger(__name__)


def sequence(name, start, end):
    logger.info("%s started at %i", name, start)
    value = start
    while value < end:
        rcvd = yield value
        value += 1
        logger.info("%s received: %s", name, rcvd)
    logger.info("%s finished at %i", name, end)
    return end


def main():
    step1 = yield from sequence("first", 0, 5)
    step2 = yield from sequence("second", step1, 10)
    return step1 + step2


if __name__ == "__main__":
    a = main()
    print(next(a))
    print(a.send("my custom value"))
