#!/usr/bin/env python

import re
import math
from collections import defaultdict, deque


with open("input.txt", "r") as f:
    content = f.read()
    LINES = content.splitlines()
    GRID  = [[c for c in row] for row in LINES]


# r, c, dr, dc
queue = deque([(0, -1, 0, 1)])
seen = set()

while len(queue) > 0:
    r, c, dr, dc = queue.popleft()

    r += dr
    c += dc

    if r not in range(len(GRID)) or c not in range(len(GRID[0])):
        continue

    char = GRID[r][c]
    
    if char == "." or (char == "-" and dc != 0) or (char == "|" and dr != 0):
        if (r, c, dr, dc) not in seen:
            seen.add((r, c, dr, dc))
            queue.append((r, c, dr, dc))

    elif char == "/":
        dr, dc = -dc, -dr
        if (r, c, dr, dc) not in seen:
            seen.add((r, c, dr, dc))
            queue.append((r, c, dr, dc))

    elif char == "\\":
        dr, dc = dc, dr
        if (r, c, dr, dc) not in seen:
            seen.add((r, c, dr, dc))
            queue.append((r, c, dr, dc))

    else:
        directions = [(1, 0), (-1, 0)] if char == "|" else [(0, 1), (0, -1)]
        
        for dr, dc in directions:
            if (r, c, dr, dc) not in seen:
                seen.add((r, c, dr, dc))
                queue.append((r, c, dr, dc))


# remove duplicate coords       
coords = {(r, c) for (r, c, _, _) in seen}

ANSWER = len(coords)

print("\nANSWER:", ANSWER)