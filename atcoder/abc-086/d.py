n,k = map(int, input().split())

def bound(x,k):
    if (x-k+1)>=0:
        return [(x-k+1, x+1)]
    else:
        return [(0,x+1), ((x-k+1)%(2*k), 2*k)]

board = [[0]*(2*k+1) for _ in range(2*k+1)]
for _ in range(n):
    x,y,c = input().split()
    x,y = map(int , [x,y])
    if c=='W':
        y += k
    x,y = map(lambda a:a%(2*k), [x,y])
    for a in [0,k]:
        for xx in bound((x+a)%(2*k),k):
            for yy in bound((y+a)%(2*k),k):
                board[xx[0]][yy[0]] += 1
                board[xx[1]][yy[1]] += 1
                board[xx[0]][yy[1]] -= 1
                board[xx[1]][yy[0]] -= 1

for j in range(2*k+1):
    for i in range(1,2*k+1):
        board[i][j] += board[i-1][j]

for i in range(2*k+1):
    for j in range(1,2*k+1):
        board[i][j] += board[i][j-1]

ans = 0
for i in range(2*k+1):
    ans = max(ans,max(board[i]))

print(ans)
