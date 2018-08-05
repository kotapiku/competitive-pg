a,b,k = map(int, input().split())

if b-a+1>=2*k:
    for n in range(a,a+k):
        print(n)
    for n in range(b-k+1,b+1):
        print(n)
else:
    for n in range(a,b+1):
        print(n)
