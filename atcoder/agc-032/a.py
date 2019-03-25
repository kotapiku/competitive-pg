input()
res = list(map(int, input().split()))
ans = []

while res:
    for i in range(len(res), 0, -1):
        if res[i-1] == i:
            ans.append(i)
            del res[i-1]
            break
        if i == 1:
            print(-1)
            exit()

for i in ans[::-1]:
    print(i)
