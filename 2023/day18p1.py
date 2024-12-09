#!/usr/bin/env python

import re
import math
from collections import defaultdict, deque


with open("input.txt", "r") as f:
    content = f.read()
    LINES = content.splitlines()
    GRID  = [[c for c in row] for row in LINES]


points = [(0, 0)]
directions = {"U": (-1, 0), "D": (1, 0), "L": (0, -1), "R": (0, 1)}

boundary_points_count = 0
for line in LINES:
    direction, steps, hex_rgb_code = line.split()
    hex_rgb_code = hex_rgb_code[2:-1]
    dr, dc = directions[direction]
    steps = int(steps)
    boundary_points_count += steps
    r, c = points[-1]
    points.append((r + dr*steps, c + dc*steps))


# Shoelace formula (https://en.wikipedia.org/wiki/Shoelace_formula)
A = 0
for i in range(len(points)):
    A += points[i][0] * (points[i - 1][1] - points[(i + 1) % len(points)][1])

A = abs(A) // 2

# Pick's theorem (https://en.wikipedia.org/wiki/Pick%27s_theorem)
interior_points_count = A - (boundary_points_count // 2) + 1

ANSWER = interior_points_count + boundary_points_count

print("\nANSWER:", ANSWER)