A = 699
B = 124

factors = [16807, 48271]
mod = 2147483647

# Part 1
count = 0
for i in range(40000000):
	A *= factors[0]
	B *= factors[1]
	A, B = A % mod, B % mod
	A_2, B_2 = bin(A)[-16:], bin(B)[-16:]
	if A_2 == B_2:
		count += 1

print(count)

# Part 2
def A(n):
	A = 699

	values_provided = 0
	while values_provided < n:
		A *= 16807
		A = A % 2147483647

		if A % 4 == 0:
			values_provided += 1
			yield bin(A)[-16:]

def B(n):
	B = 124

	values_provided = 0
	while values_provided < n:
		B *= 48271
		B = B % 2147483647

		if B % 8 == 0:
			values_provided += 1
			yield bin(B)[-16:]

n = 5000000
As = list(A(n))
Bs = list(B(n))

print(sum([ 1 for i in range(n) if As[i] == Bs[i] ]))