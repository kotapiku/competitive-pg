#include<bits/stdc++.h>
#define rep(i,n) for(int i=0;i<n;i++)
using namespace std;
 
int main(){
    cin.tie(0);
    ios::sync_with_stdio(false);
 
    int n;
    cin>>n;
    int t[n+1], x[n+1], y[n+1];
 
    rep(i,n)cin>>t[i+1]>>x[i+1]>>y[i+1];
    t[0] = 0;
    x[0] = 0;
    y[0] = 0;
 
    bool flag = true;
 
    rep(i,n){
        int a = abs(x[i+1]-x[i])+abs(y[i+1]-y[i]);
        int b = t[i+1]-t[i];
        if(a>b or (b-a)%2==1){
            flag = false;
            break;
        }
    }
    if(flag) cout<<"Yes"; else cout<<"No";
}
