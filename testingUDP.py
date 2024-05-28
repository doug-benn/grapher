import random
import socket
import time

socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

addr = ("127.0.0.1", 6969)

index = 0
randmod = 10

while True:
    index += 1
    if index % 10 == 0:
        randmod += 5

    message = f"{index} {random.randint(0,randmod)}"
    print(f"Python sent: {message.encode()}")

    socket.sendto(message.encode(), addr)
    time.sleep(0.1)
