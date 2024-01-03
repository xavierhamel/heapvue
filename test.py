import time
import random
import sys

# print("m:400,800,aa")
# print("m:450,800,aa")
# sys.stdout.flush()
# 
# while True:
#     pass
used = set()

while True:
    ptr_int = random.randrange(200000)
    ptr = hex(ptr_int)[2:]
    size = hex(random.randrange(16, 1024))[2:]
    if ptr in used:
        print("used")
        continue
    print(f"m:{ptr},{size},aa")
    used.add(ptr_int)
    sys.stdout.flush()
    time.sleep(0.1)
    if len(used) < 24:
        continue
    ptr_int = next(iter(used))
    ptr = hex(ptr_int)[2:]
    used.remove(ptr_int)
    print(f"f:{ptr},aaa")
    sys.stdout.flush()
    time.sleep(0.1)

    if random.randrange(24) == 1:
        ptr_int = random.randrange(200000)
        ptr = hex(ptr_int)[2:]
        print(f"c:{ptr},aa")
        sys.stdout.flush()
