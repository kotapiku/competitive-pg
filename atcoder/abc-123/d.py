x, y, z, k = map(int, input().split())
la = list(map(int, input().split()))
lb = list(map(int, input().split()))
lc = list(map(int, input().split()))

la.sort()
lb.sort()
lc.sort()

def overk(p):   # 合計がp以上のものがk個以上あるか
    cnt = 0
    for xa in la[::-1]:
        for xb in lb[::-1]:
            if xa + xb + lc[-1] < p:    # 枝刈り
                break
            for xc in lc[::-1]:
                if xa + xb + xc >= p:
                    cnt += 1
                else:
                    break
                if cnt >= k:
                    return True
    return False

def binarysearch(l, r):
    m = (l + r)//2 + 1
    if r <= l:
        return l
    if overk(m):
        return binarysearch(m, r)
    else:
        return binarysearch(l, m-1)

boarder = binarysearch(0, la[-1] + lb[-1] + lc[-1])

ans = []
cnt = 0
for xa in la[::-1]:
    for xb in lb[::-1]:
        if xa + xb + lc[-1] < boarder+1:
            break
        for xc in lc[::-1]:
            if xa + xb + xc >= boarder+1:
                cnt += 1
                ans.append(xa + xb + xc)
            else:
                break

ans.sort()
ans.reverse()
for x in ans:
    print(x)
num = k - len(ans)
if num > 0:
    for _ in range(num):
        print(boarder)
