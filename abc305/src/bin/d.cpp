#include <bits/stdc++.h>
using namespace std;
using ll = long long;
#define rep(i, n) for (int i = 0; i < (n); ++i)

int n;
vector<ll> a;
vector<ll> sleep;

ll sleep_time(ll t)
{
    // t の直前の 起床or就寝 の時刻
    int ok = 0;
    int ng = n;
    while (ok + 1 < ng)
    {
        int wj = (ok + ng) / 2;
        if (a[wj] <= t)
            ok = wj;
        else
            ng = wj;
    }
    ll result = sleep[ok];
    if (ok % 2 == 1) // 直前が就寝
        result += t - a[ok];
    return result;
}

int main()
{
    cin >> n; // odd
    a.resize(n);
    rep(i, n) cin >> a[i];

    sleep.resize(n);
    sleep[0] = 0;
    for (int i = 1; i < n; i++)
    {
        if (i % 2 == 0)
            sleep[i] = sleep[i - 1] + (a[i] - a[i - 1]);
        else
            sleep[i] = sleep[i - 1];
    }
    int q;
    cin >> q;
    rep(_, q)
    {
        ll l, r;
        cin >> l >> r;
        cout << sleep_time(r) - sleep_time(l) << endl;
    }
}
