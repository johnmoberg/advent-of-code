import csv
import numpy as np
import statistics

def parse_file(txt, include_weight=False):
	result = []

	for line in txt.splitlines():
		terms = line.replace(',', '').split(' ')
		parent = terms[0]
		if len(terms) <= 2:
			children = []
		else:
			children = terms[3:]

		if include_weight:
			result.append([ parent, int(terms[1][1:-1]), tuple(children) ])
		else:
			result.append([ parent, tuple(children) ])

	return result


tsv = open("day7_input.txt").read()

# Part 1
inp = parse_file(tsv)

all_ = set()
bad = set()
for i, j in inp:
	all_.add(i)
	bad.update(j)

root = next(iter(all_ - bad))
print('The bottom program is {}'.format(root))

# Part 2
inp = parse_file(tsv, True)

A = {}

# Add to dictionary
for i, weight, j in inp:
	A[i] = [ weight, j ]

def find_imbalance(node):
	children = list(map(find_imbalance, A[node][1]))

	# If there are less than three children, there can be no unique different child
	if len(children) < 3:
		return 'good', A[node][0] + sum([ child[1] for child in children ])

	m = statistics.median([ child[1] for child in children[:3] ])
	s = A[node][0]

	for j, q in enumerate(children):
		if q[0] == 'bad':
			return q
		if q[1] != m:
			# We found the erronous weight!
			missing_weight = m - q[1]
			cur_weight = A[A[node][1][j]][0]

			print('Imbalance found in {}!'.format(A[node][1][j]))
			print('Current weight: {}'.format(cur_weight))
			print('Correct weight: {}'.format(cur_weight + missing_weight))

			return 'bad', -1, A[node][1][j]
		s += q[1]

	return 'good', s

find_imbalance(root)