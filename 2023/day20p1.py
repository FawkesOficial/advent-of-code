#!/usr/bin/env python

import re
import math
from collections import defaultdict, deque


with open("input.txt", "r") as f:
    content = f.read()
    LINES = content.splitlines()
    GRID  = [[c for c in row] for row in LINES]


class Module:
    def __init__(self, name: str, type, outputs):
        self.name = name
        self.type = type
        self.outputs = outputs

        if type == "%":
            self.memory = "off"
        else:
            self.memory = {}


modules = {}
broadcast_targets = []

for line in LINES:
    left, right = line.strip().split(" -> ")
    outputs = right.split(", ")
    if left == "broadcaster":
        broadcast_targets = outputs

    else:
        type = left[0]
        name = left[1:]
        modules[name] = Module(name, type, outputs)

for name, module in modules.items():
    for output in module.outputs:
        if output in modules and modules[output].type == "&":
            modules[output].memory[name] = "low"

low = 0
high = 0
for _ in range(1000):
    low += 1
    queue = deque([("broadcaster", target, "low") for target in broadcast_targets])
    
    while len(queue) > 0:
        origin, target, pulse = queue.popleft()

        if pulse == "low":
            low += 1
        else:
            high += 1
        
        if target not in modules:
            continue
        
        module = modules[target]
        
        if module.type == "%":
            if pulse == "low":
                module.memory = "on" if module.memory == "off" else "off"
                outgoing = "high" if module.memory == "on" else "low"
                for output in module.outputs:
                    queue.append((module.name, output, outgoing))
        else:
            module.memory[origin] = pulse
            outgoing = "low" if all(state == "high" for state in module.memory.values()) else "high"
            for output in module.outputs:
                queue.append((module.name, output, outgoing))


ANSWER = low * high

print("\nANSWER:", ANSWER)