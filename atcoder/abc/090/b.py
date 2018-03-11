a,b = map(int, input().split())
ans = 0

for i in range(a,b+1):
    if i == int(str(i)[::-1]):
        ans += 1

print(ans)

