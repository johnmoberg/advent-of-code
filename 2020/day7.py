from collections import defaultdict

with open('inputs/day7.txt', 'r') as f:
    regulations = f.read().split('\n')

# Part 1
dag = defaultdict(list)

for regulation in regulations:
    s = regulation.split(' ')
    bag = ' '.join(s[:2])

    if s[4] == 'no':
        continue
    for i in range(1 + (len(s) - 4) // 5):
        dag[' '.join(s[4+4*i+1:4+4*i+3])].append(bag)

def count_children(root, children=set()):
    for child in dag[root]:
        count_children(child)
        children.add(child)

    return len(children)

print(f'Part 1: {count_children("shiny gold")}')

# Part 2
dag = defaultdict(list)

for regulation in regulations:
    s = regulation.split(' ')
    bag = ' '.join(s[:2])

    if s[4] == 'no':
        continue
    for i in range(1 + (len(s) - 4) // 5):
        dag[bag].append((int(s[4+4*i]), ' '.join(s[4+4*i+1:4+4*i+3])))

def dfs(n, root):
    s = 0
    for i, c in dag[root]:
        s += dfs(i, c)

    return n*(1 + s)

print(f'Part 2: {dfs(1, "shiny gold")-1}')