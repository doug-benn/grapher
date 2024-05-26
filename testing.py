import random
import time

for i in range(1000):
    print(f"{i} {random.randint(0,10)}")
    time.sleep(0.01)

for i in range(1000):
    print(f"{i+1000} {random.randint(0,100)}")
    time.sleep(0.01)

for i in range(1000):
    print(f"{i+2000} {random.randint(0,1000)}")
    time.sleep(0.01)
