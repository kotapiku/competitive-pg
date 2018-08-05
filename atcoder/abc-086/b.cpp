#include<bits/stdc++.h>
#define rep(i,n) for(int i=0;i<n;i++)
using namespace std;
 
int main(){
    cin.tie(0);
    ios::sync_with_stdio(false);
 
    int a,b;
    cin>>a>>b;
    int c;
    c = stoi(to_string(a)+to_string(b));
    int d = sqrt(c);
    if(d*d == c) cout<<"Yes"; else cout<<"No";
}
