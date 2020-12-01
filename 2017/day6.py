import csv
import numpy as np

def redistribute(banks):
	distributor = np.argmax(banks)
	blocks_left = banks[distributor]
	banks[distributor] = 0
	current_beneficiary = distributor + 1

	while blocks_left > 0:
		banks[current_beneficiary % len(banks)] += 1
		current_beneficiary += 1
		blocks_left -= 1

	return banks

with open("day6_input.txt") as tsv:
	l = list(csv.reader(tsv, delimiter='\t'))
	banks = list(map(int, l[0]))

seen = [ banks ]

while True:
	banks = redistribute(banks[:])

	if banks in seen:
		cycle_length = len(seen) - seen.index(banks)
		print('A configuration seen before occurred after {} redistributions.'.format(len(seen)))
		print('The cycle has length {}'.format(cycle_length))
		break
	else:
		seen.append(banks)