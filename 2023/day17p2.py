#!/usr/bin/env python

import re
import math
from collections import defaultdict, deque
from heapq import heappush, heappop


with open("input.txt", "r") as f:
    content = f.read()
    LINES = content.splitlines()
    GRID  = [[int(c) for c in row] for row in LINES]


ANSWER=None

seen = set()
prio_queue = [(0, 0, 0, 0, 0, 0)]

while len(prio_queue) > 0:
    heat_loss, r, c, dr, dc, curr_steps = heappop(prio_queue)
    
    if (r, c) == (len(GRID)-1, len(GRID[0])-1) and curr_steps >= 4:
        ANSWER = heat_loss
        break

    if (r, c, dr, dc, curr_steps) in seen:
        continue

    seen.add((r, c, dr, dc, curr_steps))
    
    if curr_steps < 10 and (dr, dc) != (0, 0):
        new_r = r + dr
        new_c = c + dc
        if new_r in range(len(GRID)) and new_c in range(len(GRID[0])):
            heappush(prio_queue, (heat_loss + GRID[new_r][new_c], new_r, new_c, dr, dc, curr_steps + 1))


    if curr_steps >= 4 or (dr, dc) == (0, 0):
        directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        for new_dr, new_dc in directions:
            if (new_dr, new_dc) != (dr, dc) and (new_dr, new_dc) != (-dr, -dc):
                new_r = r + new_dr
                new_c = c + new_dc
                if new_r in range(len(GRID)) and new_c in range(len(GRID[0])):
                    heappush(prio_queue, (heat_loss + GRID[new_r][new_c], new_r, new_c, new_dr, new_dc, 1))


print("\nANSWER:", ANSWER)