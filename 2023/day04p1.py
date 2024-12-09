#!/usr/bin/env python

ANSWER=None

with open("input.txt", "r") as f:
    lines = f.read().strip().split("\n")


points = []


for line in lines:
    winning_numbers, personal_numbers = [x.split() for x in line.split(": ")[1].split(" | ")]

    count = 0
    for num in personal_numbers:
        # print(personal_numbers)
        if num in winning_numbers:
            count += 1
    
    points.append(int(2**(count-1)))

    
    # print(line.split(": ")[0], ":", count, 2**(count-1))


ANSWER = sum(points)

print(ANSWER)