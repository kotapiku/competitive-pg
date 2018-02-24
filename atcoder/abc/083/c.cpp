#include<bits/stdc++.h>
#define rep(i,n) for(int i=0;i<n;i++)
using namespace std;

int main(){
    cin.tie(0);
    ios::sync_with_stdio(false);

    long long x,y;
    cin>>x>>y;
    int ans = 0;
    while(x<=y){
        ans += 1;
        x *= 2;
    }
    cout<<ans;
}
