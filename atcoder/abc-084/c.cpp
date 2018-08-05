#include<bits/stdc++.h>
#define rep(i,n) for(int i=0;i<n;i++)
using namespace std;

int main(){
    cin.tie(0);
    ios::sync_with_stdio(false);

    int n;
    cin>>n;
    int c[n-1],s[n-1],f[n-1];
    rep(i,n-1) cin>>c[i]>>s[i]>>f[i];

    rep(i,n){
        int t = 0;
        for(int j=i;j<n-1;j++){
            if(t<s[j]) t = s[j]+c[j];
            else t = ceil((t-s[j])*1.0/f[j])*f[j] + s[j] + c[j];
        }
        cout<<t<<"\n";
    }
}
