#include<bits/stdc++.h>
#define rep(i,n) for(int i=0;i<n;i++)
using namespace std;

int main(){
    cin.tie(0);
    ios::sync_with_stdio(false);

    int a,b,c,d;
    cin>>a>>b>>c>>d;
    int n = a+b-c-d;
    if(n>0) cout<<"Left"; else if(n==0) cout<<"Balanced"; else cout<<"Right";

}
