import csv
import itertools

with open("day4_input.txt") as tsv:
	valid = [ passphrase for passphrase in csv.reader(tsv, delimiter=" ") if len(passphrase) == len(set(passphrase)) ]
	print('{} valid passphrases for part one'.format(len(valid)))

	# Sort the letters in each word of each passphrase
	sorted_letters = [ [ ''.join(word) for word in map(sorted, passphrase) ] for passphrase in valid ]
	valid_two = [ passphrase for passphrase in sorted_letters if len(passphrase) == len(set(passphrase)) ]
	print('{} valid passphrases for part two'.format(len(valid_two)))