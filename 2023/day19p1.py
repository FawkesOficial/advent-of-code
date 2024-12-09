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


def is_accepted(item: dict, name: str = "in") -> bool:
    if name == "R":
        return False
    if name == "A":
        return True

    rules, fallback = workflows[name]
    
    for key, comp_symbol, num, target in rules:
        if comp_symbol == ">" and item[key] > num or comp_symbol == "<" and item[key] < num:
            return is_accepted(item, target)
    
    return is_accepted(item, fallback)


total = 0
for line in ratings.splitlines():
    item = {}
    for segment in line[1:-1].split(","):
        char, num = segment.split("=")
        item[char] = int(num)

    if is_accepted(item):
        total += sum(item.values())


ANSWER = total

print("\nANSWER:", ANSWER)