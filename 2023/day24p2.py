#!/usr/bin/env python

import re
import math
from collections import defaultdict, deque

import sympy


with open("input.txt", "r") as f:
    content = f.read()
    LINES = content.splitlines()
    GRID  = [[c for c in row] for row in LINES]


hailstones = [tuple(map(int, line.replace("@", ",").split(","))) for line in LINES]

x0r, y0r, z0r, v0xr, v0yr, v0zr = sympy.symbols("x0r, y0r, z0r, v0xr, v0yr, v0zr")

equations = []
for i, (x0, y0, z0, v0x, v0y, v0z) in enumerate(hailstones):
    equations.append((x0r - x0) * (v0y - v0yr) - (y0r - y0) * (v0x - v0xr))
    equations.append((y0r - y0) * (v0z - v0zr) - (z0r - z0) * (v0y - v0yr))
    
    if i < 2:
        continue

    answers = [sol for sol in sympy.solve(equations) if all(x.is_integer for x in sol.values())]
    if len(answers) == 1:
        break
    
answer = answers[0]

ANSWER = answer[x0r] + answer[y0r] + answer[z0r]

print("\nANSWER:", ANSWER)