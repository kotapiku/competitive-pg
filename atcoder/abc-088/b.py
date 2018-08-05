n = int(input())
a = list(map(int, input().split()))

alice = 0
bob = 0

a.sort()
a.reverse()

for i in range(n):
    if i%2 == 0:
        alice += a[i]
    else:
        bob += a[i]

print(alice-bob)
