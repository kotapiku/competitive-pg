#include<bits/stdc++.h>
#define rep(i,n) for(int i=0;i<n;i++)
using namespace std;
 
int main(){
    cin.tie(0);
    ios::sync_with_stdio(false);
 
    int n;
    cin>>n;
    int a[n], b[n];
 
    rep(i,n)cin>>a[i];
    rep(i,n)cin>>b[i];
 
    int max=0;
    rep(i,n){
        int m = 0;
        rep(j,n){
            if(j<i) m += a[j];else if(j==i) m += a[j]+b[j];else m += b[j];
        }
        if(max<m) max = m;
    }
    cout<<max<<"\n";
}
