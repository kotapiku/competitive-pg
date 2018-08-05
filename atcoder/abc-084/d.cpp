#include<bits/stdc++.h>
#define rep(i,n) for(int i=0;i<n;i++)
using namespace std;

bool IsPrime(int num)
{
    if (num < 2) return false;
    else if (num == 2) return true;
    else if (num % 2 == 0) return false; // 偶数はあらかじめ除く

    double sqrtNum = sqrt(num);
    for (int i = 3; i <= sqrtNum; i += 2)
    {
        if (num % i == 0)
        {
            // 素数ではない
            return false;
        }
    }

    // 素数である
    return true;
}

bool Is2017(int num)
{
    return (IsPrime(num) and IsPrime((num+1)/2));
}

int main(){
    int q;
    cin>>q;
    int prime[100000];
    prime[1] = 0;
    rep(i,100000) if(i%2!=0) prime[i] = prime[i-2] + Is2017(i); else prime[i] = prime[i-1];

    rep(i,q){
        int l,r;
        cin>>l>>r;
        cout<<prime[r] - prime[l-1]<<"\n";
    }
}
