#!/usr/bin/env python

import re
from collections import defaultdict


with open("input.txt", "r") as f:
    lines = f.read().split("\n")[:-1]


def predict(history: list[int]) -> int:
    diffs = [history]
    current_diff = history
    while any(diffs[-1]):
        
        new_diff = []
        i = 0
        while i < len(current_diff)-1:
            new_diff.append(current_diff[i+1]-current_diff[i])

            i += 1
        
        current_diff = new_diff
        diffs.append(current_diff)
    

    diffs[-2].append(diffs[-2][-1])
    
    i = len(diffs)-2
    while i > 0:
        diffs[i-1].append(diffs[i][-1]+diffs[i-1][-1])

        i -= 1

    return diffs[0][-1]


predictions = []

for line in lines:
    prediction = predict(list(map(int, line.split())))
    predictions.append(prediction)

ANSWER = sum(predictions)

print("ANSWER:", ANSWER)