#!/usr/bin/env python

ANSWER=None

with open("input.txt", "r") as f:
    matrix = f.read().strip().split("\n")


rows = len(matrix)
cols = len(matrix[0])

def get_adj(i, j, radius=2):
    adj = []
    
    m = []
    for row, vec in enumerate(matrix[max(0, i-(radius-1)):min(rows, i+radius)]):
        m.append(vec[max(0, j-(radius-1)):min(cols, j+radius)])

    # print(m)

    hits = []
    for a, x in enumerate(m):
        for b, y in enumerate(x):
            if y.isdigit():
                hits.append((i+a-1, j+b-1))

    # print(hits)

    for hit in hits:
        num = search_num(hit)
        # print(num)
        adj.append(num)



    return list(set(adj))

def search_num(coord):
    i, j = coord

    # Search Left
    k = j-1
    while (k>=0 and matrix[i][k].isdigit()):
        k -= 1
    
    left_boundary = k+1

    # Search Right
    k = j
    while (k<cols and matrix[i][k].isdigit()):
        k += 1
    
    right_boundary = k

    return int(matrix[i][left_boundary:right_boundary])

gear_ratios = []

i = 0
while (i < rows):
    j = 0

    while (j < cols):
        if matrix[i][j] == "*":            
            adj = get_adj(i, j)
            if len(adj) == 2:
                gear_ratios.append(adj[0]*adj[1])

        j +=1

    i += 1

ANSWER = sum(gear_ratios)

print("ANSWER:",ANSWER)