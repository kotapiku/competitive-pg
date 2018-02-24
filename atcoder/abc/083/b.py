n,a,b = map(int, input().split())
def souwa(x):
    ans = 0
    for i in range(1,n+1):
        l = [int(j) for j in list(str(i))]
        if sum(l) <= x:
            ans += i
    return ans 

print(souwa(b) - souwa(a-1))
