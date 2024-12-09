#!/usr/bin/env python

import re
import math
from collections import defaultdict, deque


with open("input.txt", "r") as f:
    grid = f.read().splitlines()

for i, row in enumerate(grid):
    j = row.find("S")

    if j != -1:
        start_pos = (i, j)
        break

GOES_UP     = ["S", "|", "J", "L"]
TAKES_UP    = ["|", "7", "F"]
GOES_DOWN   = ["S", "|", "7", "F"]
TAKES_DOWN  = ["S", "|", "J", "L"]
GOES_LEFT   = ["S", "-", "J", "7"]
TAKES_LEFT  = ["-", "L", "F"]
GOES_RIGHT  = ["S", "-", "L", "F"]
TAKES_RIGHT = ["-", "J", "7"]


def get_char_at(pos: tuple, grid: list[str]) -> str:
    r, c = pos
    return grid[r][c]

def get_above(pos: tuple, grid: list[str]) -> tuple:
    r, c = pos
    return (r-1, c)

def get_bellow(pos: tuple, grid: list[str]) -> tuple:
    r, c = pos
    return (r+1, c)

def get_left(pos: tuple, grid: list[str]) -> tuple:
    r, c = pos
    return (r, c-1)

def get_right(pos: tuple, grid: list[str]) -> tuple:
    r, c = pos
    return (r, c+1)

def explore(pos: tuple, seen: set, grid: list[str]) -> list:
    result = []

    rows = len(grid)
    cols = len(grid[0])
    pos_char = get_char_at(pos, grid)

    if pos[0] > 0 and pos_char in GOES_UP:
        above = get_above(pos, grid)
        if get_char_at(above, grid) in TAKES_UP and above not in seen:
            result.append(above)

    if pos[0] < rows-1 and pos_char in GOES_DOWN:
        bellow = get_bellow(pos, grid)
        if get_char_at(bellow, grid) in TAKES_DOWN and bellow not in seen:
            result.append(bellow)
    
    if pos[1] > 0 and pos_char in GOES_LEFT:
        left = get_left(pos, grid)
        if get_char_at(left, grid) in TAKES_LEFT and left not in seen:
            result.append(left)
    
    if pos[1] < cols-1 and pos_char in GOES_RIGHT:
        right = get_right(pos, grid)
        if get_char_at(right, grid) in TAKES_RIGHT and right not in seen:
            result.append(right)

    return result


explored = {start_pos}
queue = deque([start_pos])

while len(queue) > 0:
    current_pos = queue.popleft()

    result = explore(current_pos, seen=explored, grid=grid)
    for new_pos in result:
        explored.add(new_pos)
        queue.append(new_pos)


ANSWER = len(explored) // 2

print("\nANSWER:", ANSWER)