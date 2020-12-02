from collections import Counter

with open('inputs/day2.txt', 'r') as f:
    lines = f.read().split('\n')

# Part 1
n_valid = 0

for line in lines:
    dash_idx = line.find('-')
    min_occ = int(line[:dash_idx])
    max_occ = int(line[dash_idx+1:line.find(' ')])
    char = line[line.find(':')-1]
    pwd = line[line.find(':')+2:]

    n_valid += min_occ <= Counter(pwd)[char] <= max_occ

print(f'Part 1: {n_valid}')

# Part 2
n_valid = 0

for line in lines:
    dash_idx = line.find('-')
    idx_1 = int(line[:dash_idx]) - 1
    idx_2 = int(line[dash_idx+1:line.find(' ')]) - 1
    char = line[line.find(':')-1]
    pwd = line[line.find(':')+2:]

    if pwd[idx_1] == char and pwd[idx_2] == char:
        continue
    elif pwd[idx_1] == char or pwd[idx_2] == char:
        n_valid += 1

print(f'Part 2: {n_valid}')
