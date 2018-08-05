#include<bits/stdc++.h>
#define rep(i,n) for(int i=0;i<n;i++)

using namespace std;

int main(){
    cin.tie(0);
    ios::sync_with_stdio(false);

    set<char> s;
    int n;
    cin>>n;
    rep(i,n){
        char x;
        cin>>x;
        s.insert(x);
    }
    map<int,string> m;
    m[3] = "Three";
    m[4] = "Four";
    cout<<m[s.size()];
}
