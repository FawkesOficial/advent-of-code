#!/usr/bin/env python

import re
import math
from collections import defaultdict, deque


with open("input.txt", "r") as f:
    content = f.read()
    LINES = content.splitlines()
    GRID  = [[c for c in row] for row in LINES]


hailstones = [tuple(map(int, line.replace("@", ",").split(","))) for line in LINES]

class Hailstone:
    def __init__(self, x0, y0, z0, v0x, v0y, v0z):
        # Parametric Form
        self.x0 = x0
        self.y0 = y0
        self.z0 = z0
        self.v0x = v0x
        self.v0y = v0y
        self.v0z = v0z
        
        # Standard From
        self.a = v0y
        self.b = -v0x
        self.c = v0y*x0 - v0x*y0

hailstones = [Hailstone(*hailstone) for hailstone in hailstones]

total = 0
for i, hs1 in enumerate(hailstones):
    for hs2 in hailstones[:i]:
        a1, b1, c1 = hs1.a, hs1.b, hs1.c
        a2, b2, c2 = hs2.a, hs2.b, hs2.c
        
        # check if their trajectory is parallel
        if a1*b2 == b1*a2:
            continue

        x = (c1*b2 - c2*b1) / (a1*b2 - a2*b1)
        y = (c2*a1 - c1*a2) / (a1*b2 - a2*b1)
        if 200_000_000_000_000 <= x <= 400_000_000_000_000 and 200_000_000_000_000 <= y <= 400_000_000_000_000:
            if all((x - hs.x0) * hs.v0x >= 0 and (y - hs.y0) * hs.v0y >= 0 for hs in (hs1, hs2)):
                total += 1


ANSWER = total

print("\nANSWER:", ANSWER)