#!/usr/bin/env python

ANSWER=None

with open("input.txt", "r") as f:
    matrix = f.read().strip().split("\n")


nums = []

rows = len(matrix)
cols = len(matrix[0])

def hasAdjSymbol(i, j):
    for vec in matrix[max(0, i-1):min(rows, i+2)]:
        for c in vec[max(0, j-1):min(cols, j+2)]:
            if not c.isdigit() and c != ".":
                return True

    return False


i = 0
while (i < rows):
    curr_num = ""
    isValidNum = False
    j = 0

    while (j < cols):
        if matrix[i][j].isdigit():
            curr_num += matrix[i][j]
            if hasAdjSymbol(i, j):
                isValidNum = True
        elif curr_num != "":
            if isValidNum:
                nums.append(int(curr_num))

            curr_num = ""
            isValidNum = False

        j +=1

    if curr_num != "" and isValidNum:
        nums.append(int(curr_num))

    i += 1

ANSWER = sum(nums)

print("ANSWER:",ANSWER)