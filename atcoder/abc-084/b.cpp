#include<bits/stdc++.h>
#define rep(i,n) for(int i=0;i<n;i++)
using namespace std;

int main(){
    cin.tie(0);
    ios::sync_with_stdio(false);

    int a,b;
    string s;
    cin>>a>>b>>s;
    bool flag = true;
    rep(i,a) if(not isdigit(s[i])) flag = false;
    if(s[a] != '-') flag = false;
    rep(i,b) if(not isdigit(s[i+a+1])) flag = false;

    if(flag) cout<<"Yes"; else cout<<"No";
}
