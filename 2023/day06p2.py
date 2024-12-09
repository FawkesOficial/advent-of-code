#!/usr/bin/env python

import re
import sys
from collections import defaultdict
import math

with open("input.txt", "r") as f:
    lines = f.read().split("\n")[:-1]


time = int("".join(lines[0].split(": ")[1].split()))
distance = int("".join(lines[1].split(": ")[1].split()))

roots = [
    (time - math.sqrt(time*time - 4*distance)) / 2,
    (time + math.sqrt(time*time - 4*distance)) / 2
]

roots.sort()

start = math.ceil(roots[0])
end   = math.ceil(roots[1])
if end == roots[1]:
    end -= 1

num_ways = end - start

ANSWER = num_ways

print("ANSWER:", ANSWER)