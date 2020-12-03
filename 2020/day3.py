from functools import reduce
from operator import mul

with open('inputs/day3.txt', 'r') as f:
    lines = f.read().split('\n')

# Part 1
n_trees = sum([
    1 for i, row in enumerate(lines[1:]) if row[(3+3*i) % len(lines[0])] == '#'
])
print(f'Part 1: {n_trees}')

# Part 2
prod = reduce(mul, [
    sum([1 for i, row in enumerate(lines[d::d]) if row[(r+r*i) % len(lines[0])] == '#'])
    for (r,d) in [(1,1), (3,1), (5,1), (7,1), (1,2)]
])
print(f'Part 2: {prod}')