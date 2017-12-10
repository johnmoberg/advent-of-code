from functools import reduce
from operator import xor

def do_round(lengths, lst, cur_pos, skip_size):
	n = len(lst)

	for length in lengths:
		# Pick out slice
		section = lst[cur_pos:cur_pos+length]
		overflow = cur_pos + length - n

		if overflow < 0:
			overflow = 0

		extension = lst[:overflow]
		section.extend(extension)
		
		# Reverse it and replace in original list
		rev_section = section[::-1]
		lst[cur_pos:cur_pos+length-overflow] = rev_section[:length-overflow]
		lst[:overflow] = rev_section[length-overflow:]

		# Increment counters
		cur_pos = (cur_pos + length + skip_size) % n
		skip_size = skip_size + 1

	return lst, cur_pos, skip_size

# Part 1
with open('day10_input.txt') as f:
	lengths = f.readline().strip().split(',')
	lengths = map(int, lengths)

lst = list(range(0, 256))
cur_pos, skip_size = 0, 0

lst, __, __ = do_round(lengths, lst, cur_pos, skip_size)
print(lst[0] * lst[1])

# Part 2
with open('day10_input.txt') as f:
	lengths = list(map(ord, f.readline().strip()))
	lengths.extend([17, 31, 73, 47, 23])

lst = list(range(0, 256))
cur_pos, skip_size = 0, 0
for i in range(0, 64):
	lst, cur_pos, skip_size = do_round(lengths, lst, cur_pos, skip_size)

knot_hash = ''
for i in range(0, 16):
	dh_digit = hex(reduce(xor, lst[i*16:(i+1)*16]))[2:]
	dh_digit = dh_digit if len(dh_digit) > 1 else '0' + dh_digit
	knot_hash += dh_digit

print(knot_hash)