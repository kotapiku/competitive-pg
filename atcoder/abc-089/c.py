import itertools

n = int(input())
m = list("MARCH")
l = [0]*5
for _ in range(n):
    x = input()
    if x[0] in m:
        l[m.index(x[0])] += 1

ans = 0
for xs in itertools.combinations(range(5),3):
    if l[xs[0]]==0 or l[xs[1]]==0 or l[xs[2]]==0:
        a = 0
    else:
        a = 1
        for i in range(3):
            a *= l[xs[i]]
    ans += a
print(ans)

