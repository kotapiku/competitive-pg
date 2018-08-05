c = []
for _ in range(3):
    c.append(list(map(int, input().split()))) 
flag = True

na = c[0][1]-c[0][0]
nb = c[0][2]-c[0][1]
nc = c[0][0]-c[0][2]
if na+(nb+nc) != 0:
    flag = False
for i in range(1,3):
    if (c[i][1]-c[i][0]) != na or (c[i][2]-c[i][1]) != nb or (c[i][0]-c[i][2]) != nc:
        flag = False
na = c[1][0]-c[0][0]
nb = c[2][0]-c[1][0]
nc = c[0][0]-c[2][0]
for i in range(1,3):
    if (c[1][i]-c[0][i]) != na or (c[2][i]-c[1][i]) != nb or (c[0][i]-c[2][i]) != nc:
        flag = False

if flag:
    print("Yes")
else:
    print("No")
