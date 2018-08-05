n,k = map(int, input().split())
ans = 0

for b in range(1,n+1):
    if b-1>=k:
        ans += n//b*(b-k)
    if n%b>=k and n%b!=0:
        ans += n%b-k+1
    if k==0:
        ans -= 1
        if n%b==0:
            ans += 1

print(ans)

