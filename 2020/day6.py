import string

from collections import Counter

with open('inputs/day6.txt', 'r') as f:
    groups = f.read().split('\n\n')

# Part 1
everyone_count = sum(len(set(''.join(group.split('\n')))) for group in groups)
print(f'Part 1: {everyone_count}')

# Part 2
c = 0

for group in groups:
    all = Counter(''.join(group.split('\n')))
    c += sum(all[ch] == len(group.split('\n')) for ch in string.ascii_lowercase)    
print(f'Part 2: {c}')