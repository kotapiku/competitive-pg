import heapq
import math
n,h = map(int, input().split())
la = []
lb = []
for _ in range(n):
    a,b = map(int, input().split())
    la.append(a)
    lb.append(b)

la.sort()
lb.sort()
la.reverse()
lb.reverse()

ans = 0

for i in lb:
    if la[0]<i:
        h -= i
        ans += 1
        if h<=0:
            break
    else:
        break
if h>0:
    ans += math.ceil(h/la[0])
print(ans)
