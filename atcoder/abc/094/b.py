n,m,x = map(int, input().split())
l = list(map(int, input().split()))

n = 0
for i in range(m):
    if l[i]<x:
        n += 1
    else:
        break

if n<m-n:
    print(n)
else:
    print(m-n)
