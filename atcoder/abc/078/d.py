n,z,w = map(int, input().split())
a = list(map(int, input().split()))

if n!=1:
    print(max(abs(a[-1]-w),abs(a[-1]-a[-2])))
else:
    print(abs(a[0]-w))
