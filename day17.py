steps = 349

# Part one 
buffer = [0]
pos = 0

for i in range(1, 2018):
	pos = (pos + steps) % len(buffer)
	buffer = buffer[:pos+1] + [i] + buffer[pos+1:]
	pos += 1

print('The value after 2017 is {}'.format(buffer[buffer.index(2017) + 1]))

# Part two
pos = 0
value_after_zero = 0

for i in range(1, int(50e6)+1):
	pos = (pos + steps) % i
	if pos == 0:
		value_after_zero = i
	pos += 1

print('After 50 million insertions, the value after 0 is {}'.format(value_after_zero))
