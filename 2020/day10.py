with open('inputs/day10.txt', 'r') as f:
    jolts = list(map(int, f.read().split('\n')))

jolts = sorted(jolts)
assert len(jolts) == len(set(jolts))

# Part 1
one_diff = (jolts[0] == 1)
three_diff = (jolts[0] == 3) + 1
for i in range(len(jolts)-1):
    one_diff += (jolts[i+1] - jolts[i] == 1)
    three_diff += (jolts[i+1] - jolts[i] == 3)

print(f'Part 1: {one_diff * three_diff}')

# Part 2
jolts = [0] + jolts + [jolts[-1] + 3]
opt = [0] * len(jolts)
opt[0] = 1

j = 0
for i in range(1, len(jolts)):
    while jolts[i] - jolts[j] > 3:
        j += 1
    opt[i] = sum(opt[j:i])

print(f'Part 2: {opt[-1]}')
