#!/usr/bin/env python

import re
from collections import defaultdict
from itertools import combinations


with open("input.txt", "r") as f:
    grid = f.read().split("\n")[:-1]


def row_is_empty(row_idx: int) -> bool:
    return "#" not in grid[row_idx]

def col_is_empty(col_idx: int) -> bool:
    return "#" not in [grid[i][col_idx] for i in range(len(grid))]

def count_crossings(idx: int, empty_spaces: list[int]) -> int:
    return len([x for x in empty_spaces if x < idx])

def shortes_path(p1: tuple, p2: tuple) -> int:
    return abs(p2[0]-p1[0]) + abs(p2[1]-p1[1])


expansion_factor = 1000000
galaxies = []

empty_rows = [idx for idx in range(len(grid)) if row_is_empty(idx)]
empty_cols = [idx for idx in range(len(grid[0])) if col_is_empty(idx)]

for i in range(len(grid)):
    for j in range(len(grid[0])):
        if grid[i][j] == "#":
            scaled_row = i + (expansion_factor-1)*count_crossings(i, empty_rows) if count_crossings(i, empty_rows) != 0 else i
            scaled_col = j + (expansion_factor-1)*count_crossings(j, empty_cols) if count_crossings(j, empty_cols) != 0 else j

            galaxies.append((scaled_row, scaled_col))


ANSWER = sum([shortes_path(*pair) for pair in combinations(galaxies, 2)])

print("\nANSWER:", ANSWER)