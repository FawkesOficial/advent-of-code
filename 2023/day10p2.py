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

def explore(pos: tuple, s_aprox: set, seen: set, grid: list[str]) -> tuple[list, set]:
    new_nodes = []
    new_s_aprox = s_aprox

    rows = len(grid)
    cols = len(grid[0])
    pos_char = get_char_at(pos, grid)

    if pos[0] > 0 and pos_char in GOES_UP:
        above = get_above(pos, grid)
        if get_char_at(above, grid) in TAKES_UP and above not in seen:
            new_nodes.append(above)
            if pos_char == "S":
                new_s_aprox = new_s_aprox & {"|", "J", "L"}

    if pos[0] < rows-1 and pos_char in GOES_DOWN:
        bellow = get_bellow(pos, grid)
        if get_char_at(bellow, grid) in TAKES_DOWN and bellow not in seen:
            new_nodes.append(bellow)
            if pos_char == "S":
                new_s_aprox = new_s_aprox & {"|", "7", "F"}
    
    if pos[1] > 0 and pos_char in GOES_LEFT:
        left = get_left(pos, grid)
        if get_char_at(left, grid) in TAKES_LEFT and left not in seen:
            new_nodes.append(left)
            if pos_char == "S":
                new_s_aprox = new_s_aprox & {"-", "J", "7"}
    
    if pos[1] < cols-1 and pos_char in GOES_RIGHT:
        right = get_right(pos, grid)
        if get_char_at(right, grid) in TAKES_RIGHT and right not in seen:
            new_nodes.append(right)
            if pos_char == "S":
                new_s_aprox = new_s_aprox & {"-", "L", "F"}

    return new_nodes, new_s_aprox

s_aprox = {"|", "-", "J", "L", "7", "F"}

explored = {start_pos}
queue = deque([start_pos])

while len(queue) > 0:
    current_pos = queue.popleft()

    new_nodes, s_aprox = explore(current_pos, s_aprox=s_aprox, seen=explored, grid=grid)
    for new_pos in new_nodes:
        explored.add(new_pos)
        queue.append(new_pos)


clean_grid = grid.copy()
clean_grid[start_pos[0]] = clean_grid[start_pos[0]].replace("S", s_aprox.pop())
clean_grid = ["".join(ch if (r, c) in explored else "." for c, ch in enumerate(row)) for r, row in enumerate(clean_grid)]

outside = set()

for i, row in enumerate(clean_grid):
    within = False
    up = None
    for j, char in enumerate(row):
        if char == "|":
            within = not within
        elif char == "-":
            pass
        elif char in "LF":
            up = char == "L"
        elif char in "7J":
            if char != ("J" if up else "7"):
                within = not within
            up = None
        elif char == ".":
            pass

        if not within:
            outside.add((i, j))
            
            
ANSWER = len(clean_grid) * len(clean_grid[0]) - len(outside | explored)

print("\nANSWER:", ANSWER)