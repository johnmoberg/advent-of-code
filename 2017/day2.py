import csv
import itertools

# Part 1
with open("day2_input.txt") as tsv:
	checksum = sum([ max(map(int, line)) - min(map(int, line)) for line in csv.reader(tsv, delimiter="\t") ])
	print(checksum)

# Part 2
with open("day2_input.txt") as tsv:
	s = 0
	for line in csv.reader(tsv, delimiter="\t"):
		i_line = list(map(int, line))
		s += sum([ x//y for x in i_line for y in i_line if x % y == 0 and x != y ])
	
	print(s)