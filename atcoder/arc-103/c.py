import collections

n = int(input())
v = list(map(int, input().split()))
v1 = []
v2 = []

for i in range(len(v)):
    if i%2 == 0:
        v1.append(v[i])
    else:
        v2.append(v[i])

xs = sorted(collections.Counter(v1).items(), key=lambda x:-x[1])
ys = sorted(collections.Counter(v2).items(), key=lambda x:-x[1])

if xs[0][0] == ys[0][0]:
    if len(xs) == 1 and len(ys) == 1:
        print(n//2)
    elif len(xs) == 1:
        print(n//2-ys[1][1])
    elif len(ys) == 1:
        print(n//2-xs[1][1])
    else:
        print(min(n-xs[1][1]-ys[0][1], n-xs[0][1]-ys[1][1]))
else:
    print(n-max(collections.Counter(v1).values())-max(collections.Counter(v2).values()))
