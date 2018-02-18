from collections import deque
h,w = map(int, input().split()) 
s = [[1]*w for _ in range(h)]
b = 0
for i in range(h):
    a = list(input())
    for j in range(w):
        if a[j] == "#":
            s[i][j] = 0
            b += 1

d = deque([(0,0,0)])
v = [[1]*w for _ in range(h)]
v[0][0] = 0

flag = True

while d:
    x,y,a = d.popleft()
    if (x,y)==(h-1,w-1):
        print(h*w-a-1-b)
        flag = False
        break
    else:
        if 0<=x+1<h and s[x+1][y] and v[x+1][y]:
            d.append((x+1,y,a+1))
            v[x+1][y] = 0
        if 0<=x-1<h and s[x-1][y] and v[x-1][y]:
            d.append((x-1,y,a+1))
            v[x-1][y] = 0
        if 0<=y+1<w and s[x][y+1] and v[x][y+1]:
            d.append((x,y+1,a+1))
            v[x][y+1] = 0
        if 0<=y-1<w and s[x][y-1] and v[x][y-1]:
            d.append((x,y-1,a+1))
            v[x][y-1] = 0

if flag:
    print(-1)
