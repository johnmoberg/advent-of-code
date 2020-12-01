# sX = move X programs from the end to the front (s3 on abcde => cdeab)
# xA/B = swap programs at position A and B
# pA/B = makes programs named A and B swap places

def exchange(l, a, b):
	l[a], l[b] = l[b], l[a]

def partner(l, a, b):
	exchange(l, l.index(a), l.index(b))

def spin(l, n):
	l[:n], l[n:] = l[-n:], l[:-n]

def dance(programs, moves):
	for move in moves:
		if move[0] == 's':
			spin(programs, int(move[1:]))
		elif move[0] == 'x':
			arg = move[1:].split('/')
			exchange(programs, int(arg[0]), int(arg[1]))
		elif move[0] == 'p':
			arg = move[1:].split('/')
			partner(programs, move[1], move[3])

moves = open('day16_input.txt').read().split(',')
starting_programs = 'a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p'.split(',')
programs = starting_programs[:]

dances = 0
while True:
	dances += 1
	dance(programs, moves)

	if programs == starting_programs:
		print('Cycle after {} dances!'.format(dances))
		cycle_length = dances
		break

remainder = int(1e9) % cycle_length
for i in range(remainder):
	dance(programs, moves)

print('A billion dances later...')
print(''.join(programs))