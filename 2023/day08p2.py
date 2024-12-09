#!/usr/bin/env python

import re
from collections import defaultdict
import math

with open("input.txt", "r") as f:
    lines = f.read().split("\n")[:-1]


moves = [int(c) for c in lines[0].replace("R", "1").replace("L", "0")]

m = {}
for line in lines[2:]:
    current_node, _, next_node = line.split(maxsplit=2)
    
    m[current_node] = (next_node[1:4], next_node[6:-1])


starts = [node for node in m.keys() if node.endswith("A")]

step_info = {start:0 for start in starts}

for start in starts:
    i = 0
    steps = 0
    current_node = start
    while not current_node.endswith("Z"):
        ayo = moves[i%len(moves)]
        current_node = m[current_node][ayo]
        i = (i+1)%len(moves)
        steps += 1

    step_info[start] = steps

ANSWER = math.lcm(*step_info.values())

print("ANSWER:", ANSWER)