#!/usr/bin/env python

import re
from collections import defaultdict


with open("input.txt", "r") as f:
    lines = f.read().split("\n")[:-1]


steps = 0
moves = lines[0].replace("R", "1").replace("L", "0")

m = {}
for line in lines[2:]:
    current_node, _, next_node = line.split(maxsplit=2)
    
    m[current_node] = (next_node[1:4], next_node[6:-1])

i = 0 
current_node = "AAA"
while current_node != "ZZZ":
    ayo = int(moves[i%len(moves)])
    current_node = m[current_node][ayo]
    i = (i+1)%len(moves)
    steps += 1

ANSWER = steps

print("ANSWER:", ANSWER)