#!/usr/bin/env python

import re
from collections import defaultdict
from typing import Optional


with open("input.txt", "r") as f:
    patterns = [p.splitlines() for p in f.read().split("\n\n")]

def get_column(col_idx: int, grid: list[str]) -> list[str]:
    return [grid[i][col_idx] for i in range(len(grid))]

def diff_count(l1: list, l2: list) -> list:
    count = 0
    for a, b in zip(l1, l2):
        if a != b:
            count += 1
        
    return count


def find_vertical_reflection_lines(pattern: list[str], allowed_smudges: int) -> list[int]:
    refl_lines = []

    cols = len(pattern[0])

    for j in range(cols-1):
        smudge_count = 0
        k = 0
        while (0 <= j-k < cols) and (0 <= j+1+k < cols):
            diff = diff_count(get_column(j-k, pattern), get_column(j+1+k, pattern))
            if diff == 0:
                k += 1
            elif diff == 1 and smudge_count < allowed_smudges:
                smudge_count += 1
                k += 1
            else:
                break

        if j-k == -1 or j+1+k == cols and (diff == 0 or (diff == 1 and smudge_count <= allowed_smudges)):
            print("Found vertical reflection line in column", j+1)
            # print(diff)
            if smudge_count != 0:
                print("Used", smudge_count, "smudges!")


            refl_lines.append(j+1)

    return refl_lines

def find_horizontal_reflection_lines(pattern: list[str], allowed_smudges: int) -> list[int]:
    refl_lines = []
    
    rows = len(pattern)

    for i in range(rows-1):
        smudge_count = 0
        k = 0
        while (0 <= i-k < rows) and (0 <= i+1+k < rows):
            diff = diff_count(pattern[i-k], pattern[i+1+k])
            if diff == 0:
                k += 1
            elif diff == 1 and smudge_count < allowed_smudges:
                smudge_count += 1
                k += 1
            else:
                break

        if i-k == -1 or i+1+k == rows and (diff == 0 or (diff == 1 and smudge_count <= allowed_smudges)):
            print("Found horizontal reflection line in line", i+1)
            if smudge_count != 0:
                print("Used", smudge_count, "smudges!")

            refl_lines.append(i+1)

    return refl_lines

def analize(pattern: list[str]) -> int:
    result = -696969
    
    p1_v = find_vertical_reflection_lines(pattern, allowed_smudges=0)
    p1_h = find_horizontal_reflection_lines(pattern, allowed_smudges=0)

    try:
        p1_refl_line = ("vert", p1_v[0]) if len(p1_v) == 1 else ("hori", p1_h[0])
    except IndexError:

        print("\n".join(pattern))
        print()
        print("\nResults:")
        print("p1_v:", p1_v, "p1_h:", p1_h)
        print("\nERROR!")
        exit()
    
    # result = p1_refl_line[1]
    # if p1_refl_line[0] == "hori":
    #     result *= 100

    # return result

    p2_v = find_vertical_reflection_lines(pattern, allowed_smudges=1)
    p2_h = find_horizontal_reflection_lines(pattern, allowed_smudges=1)

    p1_orientation, p1_idx = p1_refl_line
    
    candidates = [x for x in [("vert", l) for l in p2_v] + [("hori", l) for l in p2_h] if x != p1_refl_line]


    if len(candidates) == 0:
        p2_refl_line = p1_refl_line
        
        # print("\n".join(pattern))
        # print()
        # print("\nResults:")
        # print("p1_refl_line:", p1_refl_line)
        # print("p2_v:", p2_v, "p2_h:", p2_h)
        # print("candidates:", candidates)
        # print("p2_ans:", p2_refl_line)
        # print("\nERROR!")
        # exit()

    else:
        p2_refl_line = candidates[0]

    # p2_orientation, p2_idx = list(candidates[0].items())[0]

    print("\nResults:")
    print("p1_refl_line:", p1_refl_line)
    print("p2_v:", p2_v, "p2_h:", p2_h)
    print("candidates:", candidates)
    print("p2_ans:", p2_refl_line)

    result = p2_refl_line[1]
    if p2_refl_line[0] == "hori":
        result *= 100

    return result



total = 0
for i, pattern in enumerate(patterns):
    print("[!] Analizing pattern", i+1)
    total += analize(pattern)
    # analize(pattern)
    print()

    # break

ANSWER = total

print("\nANSWER:", ANSWER)