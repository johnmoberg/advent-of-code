def get_seat_id(bp):
    a, b = 0, 127
    for ins in bp[:7]:
        m = (a + b) // 2

        if ins == 'F':
            b = m
        else:
            a = m + 1
    row = b
    
    a, b = 0, 7
    for ins in bp[7:]:
        m = (a + b) // 2

        if ins == 'L':
            b = m
        else:
            a = m + 1
    col = b

    return 8 * row + col

test_cases = {
    'BFFFBBFRRR': 567,
    'FFFBBBFRRR': 119,
    'BBFFBBFRLL': 820
}

for boarding_pass, seat_id in test_cases.items():
    assert get_seat_id(boarding_pass) == seat_id

with open('inputs/day5.txt', 'r') as f:
    passes = f.read().strip().split('\n')

# Part 1
max_seat_id = max([get_seat_id(bp) for bp in passes])
print(f'Part 1: {max_seat_id}')

# Part 2
seat_ids = [get_seat_id(bp) for bp in passes]
min_id, max_id = min(seat_ids), max(seat_ids)
seat_ids = set(seat_ids)

for i in range(min_id+1, max_id):
    if i in seat_ids:
        continue
    elif i-1 in seat_ids and i+1 in seat_ids:
        break

print(f'Part 2: {i}')