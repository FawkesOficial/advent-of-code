#!/usr/bin/env python

import re
import math
from collections import defaultdict, deque


with open("input.txt", "r") as f:
    content = f.read()
    LINES = content.splitlines()
    GRID  = [[c for c in row] for row in LINES]


def hax(string: str) -> int:
    current = 0
    for c in string:
        current += ord(c)
        current *= 17
        current %= 256

    return current


init_sequence = LINES[0].split(",")

total = 0
for step in init_sequence:
    total += hax(step)


ANSWER = total

print("\nANSWER:", ANSWER)