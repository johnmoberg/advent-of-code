# { opens a group.
# } closes the most recently opened group.
# Everything within < > should be ignored.
# The character following a ! should be ignored.

import re

with open('day9_input.txt') as f:
	stream = list(f)[0]

stream = re.sub('\!.', '', stream) # Remove ! and ignored character

# Part two asks how many non-canceled characters are within the garbage.
garbage_bags = 2 * stream.count('>')
len_before = len(stream)

stream = re.sub('<(.*?)>', '', stream) # Remove garbage

len_after = len(stream)
garbage_removed = len_before - len_after - garbage_bags
print('Total garbage (not including <>) is {}'.format(garbage_removed))

stream = re.sub(',', '', stream) # Remove commas

depth = 0
score = 0

for c in stream:
	if c == '{':
		depth += 1
	elif c == '}':
		score += depth
		depth -= 1

print('Total score is {}'.format(score))