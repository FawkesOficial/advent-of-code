#!/usr/bin/env python

import re
from collections import defaultdict


with open("input.txt", "r") as f:
    grid = f.read().split("\n")[:-1]
    grid = [list(row) for row in grid]


def transposed(grid: list[list[str]]) -> list[list[str]]:
    return list(map(list, zip(*grid)))

def find_max_smaller_than(idx: int, l: list) -> int:
    smallers = [x for x in sorted(l) if x < idx]
    return smallers[-1] if len(smallers) != 0 else -1

def calculate_total_load(grid: list[str]) -> int:
    total = 0
    rows = len(grid)
    cols = len(grid[0])
    for i in range(rows):
        row_round_rock_count = 0
        for j in range(cols):
            if grid[i][j] == "O":
                row_round_rock_count += 1

        total += (rows-i)*row_round_rock_count
    
    return total

rows = len(grid)
cols = len(grid[0])

columns = transposed(grid)

new_columns = []
for column in columns:
    round_rocks_idxs = []
    cube_rocks_idxs  = []

    for j, char in enumerate(column):
        if char == "O":
            round_rocks_idxs.append(j)
        elif char == "#":
            cube_rocks_idxs.append(j)

    
    new_col = column.copy()
    new_round_rocks_idxs = []

    for round_rocks_idx in round_rocks_idxs:
        new_idx = find_max_smaller_than(round_rocks_idx, cube_rocks_idxs + new_round_rocks_idxs) +1
        new_round_rocks_idxs.append(new_idx)

        print(round_rocks_idx, "-->", new_idx)

        new_col[round_rocks_idx] = "."
        new_col[new_idx] = "O"
    
    new_columns.append(new_col)

new_grid = transposed(new_columns)

print("\nNew grid:\n")
for line in new_grid:
    print("".join(line))
    

ANSWER = calculate_total_load(new_grid)

print("\nANSWER:", ANSWER)