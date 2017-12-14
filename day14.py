from functools import reduce
from operator import xor
from collections import deque

def do_round(lengths, lst, cur_pos, skip_size):
	n = len(lst)

	for length in lengths:
		# Pick out slice
		section = lst[cur_pos:cur_pos+length]
		overflow = cur_pos + length - n

		if overflow < 0:
			overflow = 0

		extension = lst[:overflow]
		section.extend(extension)
		
		# Reverse it and replace in original list
		rev_section = section[::-1]
		lst[cur_pos:cur_pos+length-overflow] = rev_section[:length-overflow]
		lst[:overflow] = rev_section[length-overflow:]

		# Increment counters
		cur_pos = (cur_pos + length + skip_size) % n
		skip_size = skip_size + 1

	return lst, cur_pos, skip_size

def hash(string):
	lengths = list(map(ord, string))
	lengths = lengths + [17, 31, 73, 47, 23]

	lst = list(range(0, 256))
	cur_pos, skip_size = 0, 0
	for i in range(0, 64):
		lst, cur_pos, skip_size = do_round(lengths, lst, cur_pos, skip_size)

	knot_hash = ''
	for i in range(0, 16):
		dh_digit = hex(reduce(xor, lst[i*16:(i+1)*16]))[2:]
		dh_digit = dh_digit if len(dh_digit) > 1 else '0' + dh_digit
		knot_hash += dh_digit

	return knot_hash

# Part 1
grid = []
string = 'hfdlxzhv'
squares = 0
for i in range(128):
	row = string + '-' + str(i)
	binary = format(int(hash(row), 16), '#0130b')[2:]
	squares += binary.count("1")
	grid.append(list(map(int, str(binary))))
print('{} squares are used.'.format(squares))

# Part 2 -- essentially to find number of connected components again
# We reuse the code from day 12
graph = {}

# First add vertices
for i in range(128):
	for j in range(128):
		if grid[i][j]:
			graph[(i,j)] = set()

# Then add edges
for (i, j) in graph.keys():
	for n in [(i-1,j), (i,j-1), (i+1,j), (i, j+1)]:
		if n in graph:
			# It's a undirected graph, so add both edges
			graph[n].add((i,j))
			graph[(i,j)].add(n)

connected_components_found = 0

while graph:
	# BFS to find connected component
	visited = []
	queue = deque()
	queue.append(list(graph)[0])
	visited.append(list(graph)[0])

	while queue:
		node = queue.popleft()
		for neighbour in graph[node]:
			if neighbour not in visited:
				queue.append(neighbour)
				visited.append(neighbour)

	for vertex in visited:
		graph.pop(vertex, None)

	#print('Connected component with {} vertices found!'.format(len(visited)))
	connected_components_found += 1

print('In total, we found {} connected components.'.format(connected_components_found))