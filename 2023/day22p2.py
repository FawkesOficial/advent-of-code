#!/usr/bin/env python

import re
import math
from collections import defaultdict, deque


with open("input.txt", "r") as f:
    content = f.read()
    LINES = content.splitlines()
    GRID  = [[c for c in row] for row in LINES]


bricks = [list(map(int, line.replace("~", ",").split(","))) for line in LINES]
bricks.sort(key=lambda brick: brick[2])

def overlaps(a: list, b: list) -> bool:
    return max(a[0], b[0]) <= min(a[3], b[3]) and max(a[1], b[1]) <= min(a[4], b[4])

for index, brick in enumerate(bricks):
    max_z = 1
    for check in bricks[:index]:
        if overlaps(brick, check):
            max_z = max(max_z, check[5]+1)
    brick[5] -= brick[2] - max_z
    brick[2] = max_z
    
bricks.sort(key=lambda brick: brick[2])

k_supports_v = {i: set() for i in range(len(bricks))}
v_supports_k = {i: set() for i in range(len(bricks))}

for j, upper in enumerate(bricks):
    for i, lower in enumerate(bricks[:j]):
        if overlaps(lower, upper) and upper[2] == lower[5]+1:
            k_supports_v[i].add(j)
            v_supports_k[j].add(i)

total = 0

for i in range(len(bricks)):
    queue = deque(j for j in k_supports_v[i] if len(v_supports_k[j]) == 1)
    falling = set(queue)
    falling.add(i)
    
    while len(queue) > 0:
        j = queue.popleft()
        for k in k_supports_v[j] - falling:
            if v_supports_k[k] <= falling:
                queue.append(k)
                falling.add(k)
    
    total += len(falling)-1


ANSWER = total

print("\nANSWER:", ANSWER)