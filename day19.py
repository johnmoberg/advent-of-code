with open("day19_input.txt") as f:
	diagram = list(map(list, f.readlines()))

pos = [0, diagram[0].index('|')]
direction = [1, 0]
seen = []
steps = 0

while True:
	tile = diagram[pos[0]][pos[1]]

	if tile == '|' or tile == '-':
		pos[0] += direction[0]
		pos[1] +=  direction[1]
	elif tile == '+':
		c = [1 - abs(direction[0]), 1 - abs(direction[1])]

		if diagram[pos[0]+c[0]][pos[1]+c[1]] != ' ':
			pos[0] += c[0]
			pos[1] += c[1]
			direction = c
		elif diagram[pos[0]-c[0]][pos[1]-c[1]] != ' ':
			pos[0] -= c[0]
			pos[1] -= c[1]
			direction = [-c[0], -c[1]]
	elif tile == ' ':
		break
	else:
		pos[0] += direction[0]
		pos[1] += direction[1]
		print('Saw {}'.format(tile))
		seen.append(tile)

	steps += 1

print(''.join(seen))
print('{} steps in total'.format(steps))