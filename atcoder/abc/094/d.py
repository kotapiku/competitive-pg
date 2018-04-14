n = int(input())
a = list(map(int, input().split()))
a.sort()

na = a[-1]

nb = na//2
ans = a[0]
for i in a[1:-1]:
    if abs(nb-i)<abs(nb-ans):
        ans = i
    ii = na-i
    if abs(nb-ii)<abs(nb-ans):
        ans = i


print(na,ans)

