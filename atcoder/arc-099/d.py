k = int(input())

def s(n):
    ans = 0
    while n!=0:
        ans += n%10
        n = n//10
    return ans

if k < 10:
    for i in range(1,k+1):
        print(i)
else:
    n = 1
    a = 1
    while k != 0:
        x = n+a
        if n*s(x) <= x*s(n):
            print(n)
            k -= 1
        else:
            n -= a
            a *= 10
        n += a
    

