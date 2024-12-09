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

label_to_number = {}
boxes = [[] for _ in range(256)]

for step in init_sequence:
    if "-" in step:
        label = step[:-1]
        index = hax(label)

        if label in boxes[index]:
            boxes[index].remove(label)

    else:
        label, length  = step.split("=")
        length = int(length)
        index = hax(label)

        if label not in boxes[index]:
            boxes[index].append(label)

        label_to_number[label] = length

total = 0

for i, box in enumerate(boxes, start=1):
    for j, label in enumerate(box, start=1):
        total += i * j * label_to_number[label]


ANSWER = total

print("\nANSWER:", ANSWER)