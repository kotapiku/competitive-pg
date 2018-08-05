#include<bits/stdc++.h>
#define rep(i,n) for(int i=0;i<n;i++)
using namespace std;

int main(){
    string s;
    cin>>s;
    int n = s.size();
    int ans = n;
    for(int i=0;i<n-1;i++){
        if(s[i]!=s[i+1]){
            ans = min(max(i+1,n-i-1),ans);
        }
    }
    cout<<ans;
}
