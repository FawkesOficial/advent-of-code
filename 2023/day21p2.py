#!/usr/bin/env python

import re
import math
from collections import defaultdict, deque


with open("input.txt", "r") as f:
    content = f.read()
    LINES = content.splitlines()
    GRID  = [[c for c in row] for row in LINES]


start_row, start_col = [(r, c) for r, row in enumerate(GRID) for c, char in enumerate(row) if char == "S"][0]

size = len(GRID)
steps = 26501365

def walk(start_row:int, start_col: int, step_count: int) -> int:
    ans = set()
    seen = {(start_row, start_col)}
    queue = deque([(start_row, start_col, step_count)])

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
    
    return len(ans)

grid_width = steps//size-1

odd_grids = (grid_width//2 * 2 + 1)**2
even_grids = ((grid_width + 1) // 2 * 2)**2

odd_points = walk(start_row, start_col, size*2 + 1)
even_points = walk(start_row, start_col, size*2)

corner_t = walk(size-1, start_col, size-1)
corner_r = walk(start_row, 0, size-1)
corner_b = walk(0, start_col, size-1)
corner_l = walk(start_row, size-1, size-1)

small_tr = walk(size-1, 0, size//2 - 1)
small_tl = walk(size-1, size-1, size//2 - 1)
small_br = walk(0, 0, size//2 - 1)
small_bl = walk(0, size-1, size//2 - 1)

large_tr = walk(size-1, 0, size*3 // 2 - 1)
large_tl = walk(size-1, size-1, size*3 // 2 - 1)
large_br = walk(0, 0, size*3 // 2 - 1)
large_bl = walk(0, size-1, size*3 // 2 - 1)


ANSWER =  odd_grids * odd_points \
        + even_grids * even_points\
        + corner_t + corner_r + corner_b + corner_l \
        + (grid_width + 1) * (small_tr + small_tl + small_br + small_bl) \
        + grid_width * (large_tr + large_tl + large_br + large_bl)

print("\nANSWER:", ANSWER)