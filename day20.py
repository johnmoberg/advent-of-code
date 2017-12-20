import numpy as np

def update(state):
	for i in range(3):
		state[1][i] += state[2][i]
		state[0][i] += state[1][i]
	return state

with open("day20_input.txt") as f:
	particles = []
	for line in f.read().splitlines():
		s = line.split(',')
		p = [int(s[0][3:]), int(s[1]), int(s[2][:-1])]
		v = [int(s[3][4:]), int(s[4]), int(s[5][:-1])]
		a = [int(s[6][4:]), int(s[7]), int(s[8][:-1])]
		particles.append([p, v, a])

s_particles = sorted(particles, key=lambda x: (np.linalg.norm(x[2]), np.linalg.norm(x[1]), np.linalg.norm(x[0])))
print('Particle {} will stay closest to the origin.'.format(particles.index(s_particles[0])))

particles = {i: state for i, state in enumerate(particles)}
no_change_iterations = 0

while True:
	to_del = set()
	for p in particles:
		particles[p] = update(particles[p])
		
		deleted = 0
		for key in particles:
			if particles[key][0] == particles[p][0] and key != p:
				to_del.add(key)
				deleted += 1
		if deleted > 0:
			to_del.add(p)

	for key in to_del:
		particles.pop(key)

	if len(to_del) > 0:
		print('{} particles deleted by collision.'.format(len(to_del)))
		no_change_iterations = 0
	else:
		no_change_iterations += 1

	if no_change_iterations > 1000:
		break

print('No collisions in 1000 iterations - {} particles left.'.format(len(particles)))