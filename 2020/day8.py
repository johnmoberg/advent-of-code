with open('inputs/day8.txt', 'r') as f:
    instructions = [(line[:3], int(line[4:])) for line in f.read().split('\n')]

# Part 1
visited = set()
pointer = 0
acc = 0

while pointer not in visited:
    visited.add(pointer)
    ins, n = instructions[pointer]

    if ins == 'nop':
        pointer += 1
    elif ins == 'acc':
        acc += n
        pointer += 1
    elif ins == 'jmp':
        pointer += n

print(f'Part 1: {acc}')

# Part 2
def does_terminate(switch_pointer):
    visited = set()
    pointer = 0
    acc = 0

    while pointer not in visited:
        if pointer == len(instructions):
            return True, acc

        visited.add(pointer)
        ins, n = instructions[pointer]

        if ins == 'nop' or (ins == 'jmp' and pointer == switch_pointer):
            pointer += 1
        elif ins == 'acc':
            acc += n
            pointer += 1
        elif ins == 'jmp' or (ins == 'nop' and pointer == switch_pointer):
            pointer += n

    return False, acc

for i, (ins, _) in enumerate(instructions):
    if ins in ['nop', 'jmp']:
        success, acc = does_terminate(i)

        if success:
            print(f'Part 2: {acc}')
            break