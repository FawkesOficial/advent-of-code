#!/usr/bin/env python

import re
import math
from collections import defaultdict


with open("input.txt", "r") as f:
    lines = f.read().splitlines()


def analyse(record: str, nums: list[int]) -> int:
    if record == "":
        return 1 if len(nums) == 0 else 0
    
    if len(nums) == 0:
        return 0 if "#" in record else 1

    result = 0

    if record[0] in [".", "?"]:
        result += analyse(record[1:], nums)
    
    if record[0] in ["#", "?"]:
        if nums[0] <= len(record) and "." not in record[:nums[0]] and (nums[0] == len(record) or record[nums[0]] != "#"):
            result += analyse(record[nums[0]+1:], nums[1:])

    return result

total = 0
for line in lines:
    record, nums = line.split()
    nums = list(map(int, nums.split(",")))

    total += analyse(record, nums)


ANSWER = total

print("\nANSWER:", ANSWER)