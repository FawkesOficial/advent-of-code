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

def find_min_greater_than(idx: int, l: list, boundary: int) -> int:
    biggers = [x for x in sorted(l) if x > idx]
    return biggers[0] if len(biggers) != 0 else boundary

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


def roll(grid: list[list[str]], direction: str) -> list[list[str]]:
    rows = len(grid)
    cols = len(grid[0])

    if direction == "north" or direction == "south":
        columns = transposed(grid)
    else:
        columns = grid

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

        round_rocks_idxs = round_rocks_idxs if (direction == "north" or direction == "west") else round_rocks_idxs[::-1]   

        for round_rocks_idx in round_rocks_idxs:
            if direction == "north" or direction == "west":
                new_idx = find_max_smaller_than(round_rocks_idx, cube_rocks_idxs + new_round_rocks_idxs) +1
            else:
                boundary = rows if direction == "south" else cols
                new_idx = find_min_greater_than(round_rocks_idx, cube_rocks_idxs + new_round_rocks_idxs, boundary) -1

            new_round_rocks_idxs.append(new_idx)

            # print(round_rocks_idx, "-->", new_idx)

            new_col[round_rocks_idx] = "."
            new_col[new_idx] = "O"
        
        new_columns.append(new_col)

    if direction == "north" or direction == "south":
        new_grid = transposed(new_columns)
    else:
        new_grid = new_columns

    return new_grid


def cycle(grid: list[list[str]]) -> list[list[str]]:
    new_grid = roll(grid, "north")
    # print("==========")
    # print("ROLL NORTH")
    # print("==========")
    # print_grid(new_grid)

    new_grid = roll(new_grid, "west")

    # print("==========")
    # print("ROLL WEST")
    # print("==========")
    # print_grid(new_grid)

    new_grid = roll(new_grid, "south")

    # print("==========")
    # print("ROLL SOUTH")
    # print("==========")
    # print_grid(new_grid)

    new_grid = roll(new_grid, "east")

    # print("==========")
    # print("ROLL EAST")
    # print("==========")
    # print_grid(new_grid)

    return new_grid

def hash_grid(grid: list[list[str]]) -> tuple:
    return tuple([tuple(row) for row in grid])

def print_grid(grid: list[list[str]]) -> None:
    print("\nNew grid:\n")
    for line in grid:
        print("".join(line))


new_grid = grid.copy()
seen = {hash_grid(grid)}
arr  = [hash_grid(grid)]
count = 0
while True:
    count += 1
    new_grid = cycle(new_grid)
    
    key = hash_grid(new_grid)

    if key in seen:
        break
    
    seen.add(key)
    arr.append(key)

first_occ_idx = arr.index(hash_grid(new_grid))

final_grid = arr[(1_000_000_000 - first_occ_idx) % (count - first_occ_idx) + first_occ_idx]


ANSWER = calculate_total_load(final_grid)

print("\nANSWER:", ANSWER)