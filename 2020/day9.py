with open('inputs/day9.txt', 'r') as f:
    numbers = list(map(int, f.read().split('\n')))

# Part 1
for i in range(25, len(numbers)):
    n_set = set(numbers[:i])

    found = False
    for j in range(i):
        if numbers[i] - numbers[j] in n_set and numbers[i] != numbers[i] // 2:
            found = True
            break
    
    if not found:
        print(f'Part 1: {numbers[i]}')
        break

# Part 2
invalid_n = numbers[i]
i, j = 0, 0
s = numbers[0]

while True:
    if s > invalid_n:
        s -= numbers[i]
        i += 1
    elif s < invalid_n:
        j += 1
        s += numbers[j]
    else:
        print(f'Part 2: {min(numbers[i:j+1])+max(numbers[i:j+1])}')
        break