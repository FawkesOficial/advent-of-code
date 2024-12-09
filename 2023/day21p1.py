#!/usr/bin/env python

import re
import math
from collections import defaultdict, deque


with open("input.txt", "r") as f:
    content = f.read()
    LINES = content.splitlines()
    GRID  = [[c for c in row] for row in LINES]


start_pos = [(r, c) for r, row in enumerate(GRID) for c, char in enumerate(row) if char == "S"][0]

ans = set()
seen = {start_pos}
queue = deque([(*start_pos, 64)])

while len(queue) > 0:
    r, c, steps = queue.popleft()

    if steps % 2 == 0:
        ans.add((r, c))
    if steps == 0:
        continue

    adjacent = [(r+1, c), (r-1, c), (r, c+1), (r, c-1)]
    for new_r, new_c in adjacent:
        if new_r in range(len(GRID)) and new_c in range(len(GRID[0])) and GRID[new_r][new_c] != "#" and (new_r, new_c) not in seen:
            seen.add((new_r, new_c))
            queue.append((new_r, new_c, steps-1))


ANSWER = len(ans)

print("\nANSWER:", ANSWER)