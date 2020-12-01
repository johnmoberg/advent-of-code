
with open('inputs/day1.txt', 'r') as f:
    numbers = list(map(int, f.read().split('\n')))

numbers_set = set(numbers)

# Part one
for n in numbers:
    if 2020 - n in numbers_set:
        print(f'Part one: {n * (2020 - n)}')
        break

# Part two
for i, a in enumerate(numbers):
    for j, b in enumerate(numbers[i+1:]):
        if 2020 - a - b in numbers_set:
            print(f'Part two: {a * b * (2020 - a - b)}')
            break