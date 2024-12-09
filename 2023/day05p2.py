#!/usr/bin/env python

import re
import sys
from collections import defaultdict


ANSWER=None

with open("input.txt", "r") as f:
    content = f.read().strip().split("\n\n")

seeds = [int(x) for x in content[0].split(": ")[1].split()]
seeds = [(seeds[i], seeds[i]+seeds[i+1]) for i in range(0, len(seeds), 2)]

levels = []
for m in content[1:]:
    ranges = []
    for line in m.split("\n")[1:]:
        dest_start, source_start, range_len = [int(x) for x in line.split()]
        ranges.append((dest_start, source_start, range_len))
    levels.append(ranges)

curr_translation = seeds
for level in levels:    
    new_translation = []
    
    while len(curr_translation) > 0:
        seed_range_start, seed_range_end = curr_translation.pop()
        
        intersected = False
        for dest_start, source_start, range_len in level:
            intersect_start = max(seed_range_start, source_start)
            intersect_end   = min(seed_range_end, source_start+range_len)

            if intersect_start < intersect_end:
                new_translation.append((intersect_start-source_start +dest_start, intersect_end-source_start +dest_start))

                # deal with leftovers
                if intersect_start > seed_range_start:
                    curr_translation.append((seed_range_start, intersect_start))
                
                if seed_range_end > intersect_end:
                    curr_translation.append((intersect_end, seed_range_end))

                intersected = True
                break

        if not intersected:
            new_translation.append((seed_range_start, seed_range_end))

    curr_translation = new_translation


ANSWER = min([rang[0] for rang in curr_translation])

print("ANSWER:", ANSWER)