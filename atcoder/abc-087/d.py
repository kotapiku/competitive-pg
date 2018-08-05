n,m = map(int, input().split())
edge = [{} for _ in range(n)]
for _ in range(m):
    a,b,c = map(int, input().split())
    edge[a-1][b-1] = c
    edge[b-1][a-1] = -c

worth = [0 for _ in range(n)]
flag = True
notvisited = set(range(n))

def dfs():
    global flag
    while(stack):
        ss,s = stack.pop()
        if s in notvisited:
            worth[s] = edge[ss][s] + worth[ss]
            notvisited.remove(s)
        else:
            if ss!=s and worth[s] != edge[ss][s] + worth[ss]:
                flag = False
                return
        for i in edge[s].keys():
            if i in notvisited:
                stack.append([s,i])


while notvisited and flag:
    s = notvisited.pop()
    stack = [[s,s]]
    dfs()
if flag:
    print("Yes")
else:
    print("No")

