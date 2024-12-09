#!/usr/bin/env python

import re
from collections import defaultdict
from typing import Optional


with open("input.txt", "r") as f:
    patterns = [p.splitlines() for p in f.read().split("\n\n")]

def get_column(col_idx: int, grid: list[str]) -> list[str]:
    return [grid[i][col_idx] for i in range(len(grid))]

def find_vertical_reflection_line(pattern: list[str]) -> Optional[int]:

    cols = len(pattern[0])
    for j in range(cols):

        k = 0
        while ((0 <= j-k+1 < cols) and (0 <= j+k < cols)) and get_column(j-k+1, pattern) == get_column(j+k, pattern):
            k += 1

        # print("j:", j, "k:", k)

        if j-k+1 == -1 or j+k == cols:
            print("Found vertical reflection line in column", j+1)

            return j+1


def find_horizontal_reflection_line(pattern: list[str]) -> int:

    rows = len(pattern)
    for i in range(rows):

        k = 0
        while ((0 <= i-k+1 < rows) and (0 <= i+k < rows)) and pattern[i-k+1] == pattern[i+k]:
            k += 1

        if i-k+1 == -1 or i+k == rows:
            print("Found horizontal reflection line in line", i+1)

            return i+1
    

def analize(pattern: list[str]) -> int:
    v_ref_l = find_vertical_reflection_line(pattern)

    return v_ref_l if v_ref_l is not None else 100*find_horizontal_reflection_line(pattern)

total = 0
for i, pattern in enumerate(patterns):
    print("[!] Analizing pattern", i+1)
    total += analize(pattern)
    print()

    # break

ANSWER = total

print("\nANSWER:", ANSWER)