#include<bits/stdc++.h>
#define rep(i,n) for(int i=0;i<n;i++)
using namespace std;

int main(){
    int h,w,d;
    cin>>h>>w>>d;
    int ax[h*w];
    int ay[h*w];
    rep(i,h) rep(j,w){
        int n;
        cin>>n;
        ax[n-1] = i;
        ay[n-1] = j;
    }

    int ld[h*w];
    rep(i,d) ld[i] = 0;
    for(int i=d;i<h*w;i++){
        ld[i] = ld[i-d] + abs(ax[i]-ax[i-d]) + abs(ay[i]-ay[i-d]);
    }

    int q;
    cin>>q;
    rep(i,q){
        int l,r;
        cin>>l>>r;
        cout<<ld[r-1]-ld[l-1]<<"\n";
    }
}
