t =int(input())
for j in range(t):
    n = int(input())
    v = list(map(int, input().split()))
    v1 = v[::2]
    v2 = v[1::2]
    v1.sort()
    v2.sort()

    flag = True
    if len(v)%2==0:
        for i in range(len(v1)):
            if v1[i]>v2[i]:
                n = 2*i
                flag = False
                break
            elif (i!=len(v1)-1 and v2[i]>v1[i+1]):
                n = 2*i+1
                flag = False
                break

    else:
        for i in range(len(v2)):
            if v1[i]>v2[i]:
                n = 2*i
                flag = False
                break
            elif v2[i]>v1[i+1]:
                n = 2*i+1
                flag = False
                break

    if flag:
        print("Case #{}: OK".format(j+1))
    else:
        print("Case #{}: {}".format(j+1,n))
