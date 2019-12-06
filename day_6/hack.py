# An easy solution to Day 6, in python

import networkx as nx

with open("input.txt") as flines:
    edges = [line.strip().split(")") for line in flines]

tree = nx.from_edgelist(edges)

path_lengths = 0
for node in tree.nodes():
    # print(node)
    path_length = len(nx.shortest_path(tree, source="COM", target=node)) - 1
    path_lengths += path_length

print(f"Number of orbits: {path_lengths}")

you_san_path = nx.shortest_path(tree, source="YOU", target="SAN")
print(f"Number of transfers required to get YOU to SAN: {len(you_san_path) -3}")