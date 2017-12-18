from collections import deque

instructions = open('day18_input.txt').read().splitlines()
names = set([ instruction[4:5] for instruction in instructions ])

# Part 1
last_played = 0
registers = { name: 0 for name in names if name.isalpha() }

i = 0
while 0 <= i < len(instructions):
	ins = instructions[i].split(' ')
	c = ins[0]
	offset = 1

	if c == 'snd':
		last_played = registers[ins[1]]
	elif c == 'rcv':
		if last_played != 0:
			print('Recovered {}'.format(last_played))
			break
	else:
		if ins[2] in registers:
			value = registers[ins[2]]
		else:
			value = int(ins[2])

		if c == 'set':
			registers[ins[1]] = value
		elif c == 'add':
			registers[ins[1]] += value
		elif c == 'mul':
			registers[ins[1]] *= value
		elif c == 'mod':
			registers[ins[1]] = registers[ins[1]] % value
		elif c == 'jgz':
			if ins[1] in registers and registers[ins[1]] > 0:
				offset = value
			elif ins[1] not in registers and int(ins[1]) > 0:
				offset = value

	i += offset


# Part 2
registers = { name: [0, 0] for name in names if name.isalpha() }
registers['p'] = [0, 1]

sent = [deque(), deque()]
i = [0, 0]
wait = [ False, False ]

how_many_times_did_program_1_send_a_value = 0

while 0 <= i[0] < len(instructions) and 0 <= i[1] < len(instructions):
	if wait == [ True, True ]:
		break

	for j in [0, 1]:
		ins = instructions[i[j]].split(' ')
		c = ins[0]
		offset = 1

		if c == 'snd':
			sent[j].append(registers[ins[1]][j])

			if j == 1:
				how_many_times_did_program_1_send_a_value += 1
		elif c == 'rcv':
			if sent[1-j]:
				registers[ins[1]][j] = sent[1-j].popleft()
				wait[j] = False
			else:
				offset = 0
				wait[j] = True
		else:
			if ins[2] in registers:
				value = registers[ins[2]][j]
			else:
				value = int(ins[2])

			if c == 'set':
				registers[ins[1]][j] = value
			elif c == 'add':
				registers[ins[1]][j] += value
			elif c == 'mul':
				registers[ins[1]][j] *= value
			elif c == 'mod':
				registers[ins[1]][j] = registers[ins[1]][j] % value
			elif c == 'jgz':
				if ins[1] in registers and registers[ins[1]][j] > 0:
					offset = value
				elif ins[1] not in registers and int(ins[1]) > 0:
					offset = value
		i[j] += offset

print('Program 1 sent a value {} times!'.format(how_many_times_did_program_1_send_a_value))