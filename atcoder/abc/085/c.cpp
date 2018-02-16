#include<bits/stdc++.h>
#define rep(i,n) for(int i=0;i<n;i++)
using namespace std;

int main(){
    cin.tie(0);
    ios::sync_with_stdio(false);

    int n, y;
    cin>>n>>y;
    int a,b;
    a = y/10000;
    b = (y%10000)/5000;

    int aa,bb,cc;
    aa = -1;
    bb = -1;
    cc = -1;

    rep(i,a+1){
        rep(j,b+(a-i)*2+1){
            int tc = (y-i*10000-j*5000)/1000;
            if((i+j+tc)==n){
                aa = i;
                bb = j;
                cc = tc;
            }
        }
    }
    cout<<aa<<" "<<bb<<" "<<cc;

}
