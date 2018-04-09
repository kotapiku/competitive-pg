import math

def total(p):
    s = 1
    t = 0
    for a in p:
        if a=='C':
            s *= 2
        else:
            t += s
    return t

t = int(input())
for i in range(t):
    d,p = input().split()
    d = int(d)
    p = list(p)
    c = 0
    d = total(p)-d  #あといくつ減らさないとけないか
    if d<=0:
        p = []
    while c!=-1 and p:
        if 'C' not in p or 'S' not in p:  #もう減らせない
            if d>0:
                c = -1
        else:
            p = p[p.index('C'):len(p)-p[::-1].index('S')]
            if 'C' not in p or 'S' not in p:
                if d>0:
                    c = -1
            else:
                n = p[::-1].index('C')
                s = 2**(p.count('C')-1)
                if d<=s*n:
                    c += math.ceil(d/s)
                    break
                else:
                    d -= s*n
                    c += n
                    del p[len(p)-n-1]


    if c == -1:
        print("Case #{}: IMPOSSIBLE".format(i+1))
    else:
        print("Case #{}: {}".format(i+1,c))

