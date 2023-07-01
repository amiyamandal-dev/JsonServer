import fire
import time


def hello(name="World"):
    time.sleep(10)
    return "Hello %s!" % name


if __name__ == "__main__":
    fire.Fire(hello)
