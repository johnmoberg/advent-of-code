with open('inputs/day11.txt', 'r') as f:
    M = [list(line) for line in f.read().split('\n')]

n_rows, n_cols = len(M), len(M[0])
directions = [
    (-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)
]

def count_adjacent(M_, i, j, part_one):
    if part_one:
        return sum(
            M_[i+a][j+b] == '#' for a, b in directions if 0 <= i+a < n_rows and 0 <= j+b < n_cols
        )
    else:
        n_adjacent = 0
        for (dx, dy) in directions:
            a, b = i, j
            while 0 <= a + dx < n_rows and 0 <= b + dy < n_cols:
                a += dx
                b += dy
                
                if M_[a][b] == '#':
                    n_adjacent += 1
                    break
                elif M_[a][b] == 'L':
                    break
        
        return n_adjacent

def count_occupied(M_, part_one):
    while True:
        changed = False
        new_M = [[M_[i][j] for j in range(n_cols)] for i in range(n_rows)]

        for i in range(n_rows):
            for j in range(n_cols):
                if M_[i][j] == '.':
                    continue
                
                n_adjacent = count_adjacent(M_, i, j, part_one)

                if M_[i][j] == 'L' and n_adjacent == 0:
                    new_M[i][j] = '#'
                    changed = True
                elif M_[i][j] == '#' and n_adjacent >= 4 + (1-int(part_one)):
                    new_M[i][j] = 'L'
                    changed = True

        M_ = new_M
        if not changed:
            break

    s = sum(M_[i][j] == '#' for i in range(n_rows) for j in range(n_cols))
    return s

print(f'Part 1: {count_occupied(M, part_one=True)}')
print(f'Part 2: {count_occupied(M, part_one=False)}')