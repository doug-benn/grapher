import random
import time

index = 0
randmod = 10
while True:
    index += 1

    if index % 10 == 0:
        randmod += 5

    print(f"{index} {random.randint(0,randmod)}")
    time.sleep(0.1)

    # for i in range(500):
    #     print(f"{index} {random.randint(0,100)}")
    #     time.sleep(0.01)

    # for i in range(500):
    #     print(f"{index} {random.randint(0,1000)}")
    #     time.sleep(0.01)
