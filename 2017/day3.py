def move_to(n):
	# Start at number 2 at position (1, 0)
	current_n = 2
	x, y = 1, 0
	current_square = 1 # We store which square in the spiral we are in

	directions = ['UP', 'LEFT', 'DOWN', 'RIGHT']
	current_direction = 0
	moves_with_current_direction = 1 # How many moves are left

	# Move in spiral
	while current_n < n:
		if moves_with_current_direction == 0:
			# Cycle through directions
			current_direction = (current_direction + 1) % 4

			if current_direction == 0: # Check if we moved to a new square
				current_square += 1
				moves_with_current_direction = 2 * current_square - 1
			else:
				moves_with_current_direction = 2 * current_square

				# If we're moving to the right, we're going to take an extra step
				if current_direction == 3:
					moves_with_current_direction += 1

		# Move according to current direction
		if directions[current_direction] == 'UP':
			y += 1
		elif directions[current_direction] == 'DOWN':
			y -= 1
		elif directions[current_direction] == 'RIGHT':
			x += 1
		else:
			x -= 1

		moves_with_current_direction -= 1
		current_n += 1

	print('Ended up at {} with coordinates ({}, {})'.format(n, x, y))
	print('Manhattan distance to (0, 0) is {}'.format(abs(x)+abs(y)))

def neighbors(cell):
    (x, y) = cell
    return [(x-1, y-1), (x, y-1), (x+1, y-1),
            (x-1, y),             (x+1, y),
            (x-1, y+1), (x, y+1), (x+1, y+1)]

def part_two(n):
	# Start at number 2 at position (1, 0)
	world = { (0,0): 1, (1,0): 1 }
	current_square = 1 # We store which square in the spiral we are in
	x, y = 1, 0

	directions = ['UP', 'LEFT', 'DOWN', 'RIGHT']
	current_direction = 0
	moves_with_current_direction = 1 # How many moves are left

	# Move in spiral
	while True:
		if moves_with_current_direction == 0:
			# Cycle through directions
			current_direction = (current_direction + 1) % 4

			if current_direction == 0: # Check if we moved to a new square
				current_square += 1
				moves_with_current_direction = 2 * current_square - 1
			else:
				moves_with_current_direction = 2 * current_square

				# If we're moving to the right, we're going to take an extra step
				if current_direction == 3:
					moves_with_current_direction += 1
					
		# Move according to current direction
		if directions[current_direction] == 'UP':
			y += 1
		elif directions[current_direction] == 'DOWN':
			y -= 1
		elif directions[current_direction] == 'RIGHT':
			x += 1
		else:
			x -= 1

		value = sum([ world[n] for n in neighbors((x,y)) if n in world ])
		world[(x,y)] = value

		if value > n:
			print('First value written greater than {} is {}'.format(n, value))
			break

		moves_with_current_direction -= 1