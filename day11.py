"""
This could be done much more easily and more efficiently using a smart coordinate
system, but I already did most of this graph solution before realizing that, so I'll
just leave it like this...
"""

import math
from collections import deque
import copy

with open('day11_input.txt') as f:
	moves = list(f.readline().strip().split(','))

# All possible directions
directions = { 'n': (0.0, 1.0),
			   'ne': (0.5, 0.5),
			   'se': (0.5, -0.5),
			   's': (0.0, -1.0),
			   'sw': (-0.5, -0.5),
			   'nw': (-0.5, 0.5) } 

# Final position of child process
moves = [ directions[move] for move in moves ]
x, y = list(map(sum, zip(*moves)))

# Create graph covering entire trajectory
# First add all nodes visited by process
vertices = { (0.0, 0.0): set() }
current_position = (0.0, 0.0)
for (dx, dy) in moves:
	current_position = (current_position[0] + dx, current_position[1] + dy)
	vertices[current_position] = set()

vertices_visited = copy.deepcopy(vertices)

# Fill in the gaps up to maximal x- and y- value
x_max = max([ abs(node[0]) for node in vertices ])
y_max = max([ abs(node[1]) for node in vertices ])
vertices.update({ (i + 0.5 if j%2 != 0 else float(i), float(j/2)): set()
				  for i in range(-math.floor(x_max), math.ceil(x_max))
				  for j in range(-math.floor(2*y_max), math.ceil(2*y_max)) })

# Add edges
for vertex in vertices:
	for (dx, dy) in directions.values():
		to = (vertex[0] + dx, vertex[1] + dy)
		if to in vertices:
			vertices[vertex].add(to)

# BFS from origin to all nodes in the graph
distances = {}
distances[(0.0, 0.0)] = 0
queue = deque()
queue.append((0.0, 0.0))

while queue:
	node = queue.popleft()
	for neighbour in vertices[node]:
		if neighbour not in distances:
			queue.append(neighbour)
			distances[neighbour] = distances[node] + 1

print("Part 1: shortest path to final position {} has distance {}.".format((x, y), distances[(x, y)]))

max_reached = max([ distances[vertex] for vertex in vertices_visited ])
print("Furthest distance reached was {}.".format(max_reached))