#!/usr/bin/env python

import re
import math
from collections import defaultdict, deque


with open("input.txt", "r") as f:
    content = f.read()
    LINES = content.splitlines()
    GRID  = [[c for c in row] for row in LINES]


start = (0, GRID[0].index("."))
end   = (len(GRID)-1, GRID[-1].index("."))

points = [start, end]

for r, row in enumerate(GRID):
    for c, char in enumerate(row):
        if char == "#":
            continue

        adjacent = [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
        neighbors = 0
        for new_row, new_col in adjacent:
            if new_row in range(len(GRID)) and new_col in range(len(GRID[0])) and GRID[new_row][new_col] != "#":
                neighbors += 1

        if neighbors >= 3:
            points.append((r, c))

graph = {point: {} for point in points}

directions = {
    "^": [(-1, 0)],
    "v": [(1, 0)],
    "<": [(0, -1)],
    ">": [(0, 1)],
    ".": [(-1, 0), (1, 0), (0, -1), (0, 1)],
}

for start_row, start_col in points:
    stack = [(start_row, start_col, 0)]
    seen = {(start_row, start_col)}

    while len(stack) > 0:
        r, c, steps = stack.pop()
        
        if steps != 0 and (r, c) in points:
            graph[(start_row, start_col)][(r, c)] = steps
            continue

        for dr, dc in directions["."]:
            new_row = r + dr
            new_col = c + dc
            if new_row in range(len(GRID)) and new_col in range(len(GRID[0])) and GRID[new_row][new_col] != "#" and (new_row, new_col) not in seen:
                stack.append((new_row, new_col, steps+1))
                seen.add((new_row, new_col))

seen = set()

def find_longest_path(point: tuple) -> int:
    global seen

    if point == end:
        return 0

    m = -float("inf")

    seen.add(point)
    for node in graph[point]:
        if node not in seen:
            m = max(m, find_longest_path(node) + graph[point][node])
    seen.remove(point)

    return m


ANSWER = find_longest_path(start)

print("\nANSWER:", ANSWER)