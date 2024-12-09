#!/usr/bin/env python

import re
import math
from collections import defaultdict, deque

import networkx as nx


with open("input.txt", "r") as f:
    content = f.read()
    LINES = content.splitlines()
    GRID  = [[c for c in row] for row in LINES]


graph = nx.Graph()

for line in LINES:
    left, right = line.split(":")
    for node in right.strip().split():
        graph.add_edge(left, node)
        graph.add_edge(node, left)

graph.remove_edges_from(nx.minimum_edge_cut(graph))
group1, group2 = nx.connected_components(graph)


ANSWER = len(group1) * len(group2)

print("\nANSWER:", ANSWER)