#!/usr/bin/env python

import re
import math
from collections import defaultdict, deque


with open("input.txt", "r") as f:
    content = f.read()
    LINES = content.splitlines()
    GRID  = [[c for c in row] for row in LINES]


workflows_info, ratings = content.split("\n\n")

workflows = {}
for line in workflows_info.splitlines():
    name, rest = line[:-1].split("{")
    rules = rest.split(",")
    fallback = rules.pop()
    workflows[name] = ([], fallback)
    for rule in rules:
        comparison, target = rule.split(":")
        key = comparison[0]
        comp_symbol = comparison[1]
        num = int(comparison[2:])
        workflows[name][0].append((key, comp_symbol, num, target))


def count_accepted(ranges: dict, name: str = "in") -> int:
    if name == "R":
        return 0

    if name == "A":
        product = 1
        for low, high in ranges.values():
            product *= high - low + 1

        return product
    
    rules, fallback = workflows[name]

    total = 0
    for key, comp_symbol, num, target in rules:
        low, high = ranges[key]
        if comp_symbol == "<":
            true_half  = (low, min(num - 1, high))
            false_half = (max(num, low), high)

        else:
            true_half  = (max(num + 1, low), high)
            false_half = (low, min(num, high))

        if true_half[0] <= true_half[1]:
            copy = dict(ranges)
            copy[key] = true_half
            total += count_accepted(copy, target)

        if false_half[0] <= false_half[1]:
            ranges = dict(ranges)
            ranges[key] = false_half

        else:
            break

    else:
        total += count_accepted(ranges, fallback)
            
    return total


ANSWER = count_accepted({key: (1, 4000) for key in "xmas"})

print("\nANSWER:", ANSWER)
