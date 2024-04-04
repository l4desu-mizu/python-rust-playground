class MyFancyStuff:

    def __init__(self, item):
        self._item = item

    def do_stuff(self):
        return self._item

    def print_stuff(self):
        print(self._item)


if __name__ == "__main__":
    MyFancyStuff("Hello").print_stuff()
    MyFancyStuff("World").print_stuff()
