#!/usr/bin/env python

import re
import sys
from collections import defaultdict


ANSWER=None

with open("input.txt", "r") as f:
    content = f.read().strip().split("\n\n")

seeds = [int(x) for x in content[0].split(": ")[1].split()]

levels = []
for m in content[1:]:
    ranges = []
    for line in m.split("\n")[1:]:
        dest_start, source_start, range_len = [int(x) for x in line.split()]
        ranges.append(
            {
                "source": range(source_start, source_start+range_len),
                "dest": range(dest_start, dest_start+range_len)
            }
        )
    levels.append(ranges)

def translate(idee, source_range, dest_range):
    return idee - source_range.start + dest_range.start if idee in source_range else idee

curr_translation = seeds
for level in levels:    
    new_translation = []
    for idee in curr_translation:
        translation = idee

        for rang in level:
            if idee in rang["source"]:
                translation = idee - rang["source"].start + rang["dest"].start
        
        # print(f"Mapped {idee} into {translation}")
        new_translation.append(translation)
        
    curr_translation = new_translation
    # print(curr_translation)

ANSWER = min(curr_translation)

print("ANSWER:", ANSWER)