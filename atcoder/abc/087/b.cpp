#include<bits/stdc++.h>
using namespace std;

int main(){
    cin.tie(0);
    ios::sync_with_stdio(false);

    int a,b,c,x;
    cin>>a>>b>>c>>x;
    int count=0;
    int na,nb,nc;
    na = x/500;
    nb = (x%500)/100;
    nc = (x%100)/50;
    for(int i=0;i<=na;i++){
        for(int j=0;j<=nb+5*i;j++){
            if(a>=(na-i) && b>=(nb+5*i-j) && c>=(nc+2*j)) count += 1;
        }
    }
    cout<<count<<endl;
}
