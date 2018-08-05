n = int(input())
l = list(map(int, input().split()))

ll = sorted(l)

nx = n//2-1
x = ll[nx]

for i in range(n):
    if x<l[i]:
        print(ll[nx])
    else:
        print(ll[nx+1])
        

