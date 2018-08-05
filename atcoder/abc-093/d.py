import math
q = int(input())
for _ in range(q):
    a,b = map(int, input().split())
    if a>b:
        a,b = b,a
    if a==b or b == a+1:
        print(2*a-2)
    else:
        c = int(math.sqrt(a*b))
        if c*c==a*b:
            c -= 1
        if c*(c+1)<a*b:
            print(2*c-1)
        else:
            print(2*c-2)

