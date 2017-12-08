import csv
lines = open('day8_input.txt').read().splitlines()
lines = [ line.split(' ') for line in lines ]

register = { line[0]: 0 for line in lines }
max_value = 0

for line in lines:
	modifee = line[0]
	value = int(line[2])
	modification = value if line[1] == 'inc' else -value
	condition = 'register["{}"]'.format(line[4]) + ''.join(line[5:])
	
	if eval(condition):
		register[modifee] += modification

		if register[modifee] > max_value:
			max_value = register[modifee]

print(max(register.values()))
print(max_value)