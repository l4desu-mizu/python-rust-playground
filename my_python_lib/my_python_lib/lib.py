import sys
import socket
import threading

class MyFancyStuff:

    def __init__(self, item1, item2):
        print(sys.executable)
        self._item1 = item1
        self._item2 = item2

    def do_stuff(self):
        return self._item1

    def print_stuff(self):
        with socket.socket() as s_in, socket.socket() as s_out:
            s_in.bind(("127.0.0.1",0))
            s_in.listen()
            t = threading.Thread(target=run_recv, args=[s_out,s_in.getsockname()])
            t.start()
            s_write, _addr = s_in.accept()
            s_write.send(self._item2.encode("utf8"))
            s_write.close()
            t.join()

def run_recv(sock: socket.socket, addr):
    sock.connect(addr)
    res = ""
    while True:
        new=sock.recv(1024).decode("utf8")
        if new:
            res+=new
        else:
            break
    print(res)

if __name__ == "__main__":
    stuff = MyFancyStuff("Hello", "World")
    print(stuff.do_stuff())
    stuff.print_stuff()
