x = int(input())
a = x//11
if a*11 == x:
    print(a*2);
elif a*11+6 >= x:
    print(a*2+1);
else:
    print(a*2+2);
