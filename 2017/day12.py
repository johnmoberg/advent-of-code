from collections import deque

graph = {}

with open('day12_input.txt') as f:
	for line in f.readlines():
		prep = line.strip().replace(',', '').split(' ')
		vertex = int(prep[0])
		edges = list(map(int, prep[2:]))
		graph[vertex] = edges

# We'll find all connected components and count them.
connected_components_found = 0

while graph:
	# BFS to find connected component
	visited = []
	queue = deque()
	queue.append(list(graph)[0])

	while queue:
		node = queue.popleft()
		for neighbour in graph[node]:
			if neighbour not in visited:
				queue.append(neighbour)
				visited.append(neighbour)

	for vertex in visited:
		graph.pop(vertex, None)

	print('Connected component with {} vertices found!'.format(len(visited)))
	connected_components_found += 1

print('In total, we found {} connected components.'.format(connected_components_found))