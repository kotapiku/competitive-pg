a,b,c = sorted(list(map(int, input().split())))

n = 0
if b!=a:
    if (b-a)%2==0:
        n += (b-a)//2
    else:
        b += 1
        c += 1
        n += 1
        n += (b-a)//2
if c!=b:
    n += c-b
        
print(n)
