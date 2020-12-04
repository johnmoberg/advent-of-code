import string

with open('inputs/day4.txt', 'r') as f:
    passports = [s.replace('\n', ' ').split(' ') for s in f.read().split('\n\n')]

# Part 1
required_fields = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}
n_valid = 0

for passport in passports:
    existing_fields = {s[:3] for s in passport}
    n_valid += all(f in existing_fields for f in required_fields)

print(f'Part 1: {n_valid}')

# Part 2
eye_colors = {'amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'}
hexdigits = set(string.hexdigits)

def is_valid(field, s):
    if field == 'byr':
        return s.isdigit() and 1920 <= int(s) <= 2002
    elif field == 'iyr':
        return s.isdigit() and 2010 <= int(s) <= 2020
    elif field == 'eyr':
        return s.isdigit() and 2020 <= int(s) <= 2030
    elif field == 'hgt':
        valid_cm = s[-2:] == 'cm' and s[:-2].isdigit() and 150 <= int(s[:-2]) <= 193
        valid_in = s[-2:] == 'in' and s[:-2].isdigit() and 59 <= int(s[:-2]) <= 76
        return valid_cm or valid_in
    elif field == 'hcl':
        return s[0] == '#' and len(s) == 7 and all(c in hexdigits for c in s[1:])
    elif field == 'ecl':
        return s in eye_colors
    elif field == 'pid':
        return s.isdigit() and len(s) == 9
    else:
        return True

n_valid = 0

for passport in passports:
    existing_fields = {s[:3]: s[4:] for s in passport}
    n_valid += all(f in existing_fields and is_valid(f, existing_fields[f]) for f in required_fields)

print(f'Part 2: {n_valid}')
