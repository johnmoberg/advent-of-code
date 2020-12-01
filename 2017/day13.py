with open('day13_input.txt') as f:
    firewall = [ list(map(int, line.strip().replace(' ', '').split(':')))
                 for line in f.readlines() ]

def special_modulo(n, mod):
    if mod == 1:
        return 1

    elems = list(range(1, mod)) + list(range(mod, 1, -1))
    ind = (n - 1) % (2 * (mod - 1))
    return elems[ind]


# Part 1
severity = 0
firewall_position = 0

for i in range(firewall[-1][0] + 1):
    if i == firewall[firewall_position][0]: # There's a scanner!
        scanner = special_modulo(i + 1, firewall[firewall_position][1])
        if scanner == 1:
            severity += i * firewall[firewall_position][1]

        firewall_position += 1

print('The severity of the trip is {}.'.format(severity))

# Part 2
caught = True
delay = 0

while caught:
    delay += 1
    caught = False
    firewall_position = 0 

    for i in range(firewall[-1][0] + 1):
        if i == firewall[firewall_position][0]: # There's a scanner!
            scanner = special_modulo(i + 1 + delay, firewall[firewall_position][1])
            if scanner == 1: # Caught!
                caught = True
                break

            firewall_position += 1

print('Delay {} worked!'.format(delay))