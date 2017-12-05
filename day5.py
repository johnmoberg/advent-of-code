import csv

with open("day5_input.txt") as tsv:
	instructions = [ int(term[0]) for term in csv.reader(tsv) ]

current_position = 0
steps = 0

while current_position < len(instructions):
	previous_position = current_position
	current_position += instructions[current_position]
	
	if instructions[previous_position] >= 3:
		instructions[previous_position] -= 1
	else:
		instructions[previous_position] += 1
	
	steps += 1

print(steps)