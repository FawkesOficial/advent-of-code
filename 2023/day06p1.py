#!/usr/bin/env python

import re
import sys
from collections import defaultdict
import math

with open("input.txt", "r") as f:
    lines = f.read().split("\n")[:-1]


times = list(map(int, lines[0].split(": ")[1].split()))
distances = list(map(int, lines[1].split(": ")[1].split()))

ways = []
for allowed_time, record_distance in zip(times, distances):
    roots = [
        (allowed_time - math.sqrt(allowed_time*allowed_time - 4*record_distance)) / 2,
        (allowed_time + math.sqrt(allowed_time*allowed_time - 4*record_distance)) / 2
    ]

    roots.sort()

    start = math.ceil(roots[0])
    end   = math.ceil(roots[1])
    if end == roots[1]:
        end -= 1

    num_ways = end - start

    if num_ways != 0:
        ways.append(num_ways)

mult = 1
for way in ways:
    mult *= way

ANSWER = mult

print("ANSWER:", ANSWER)