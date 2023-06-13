#include <bits/stdc++.h>
using namespace std;
using ll = long long;
#define rep(i, n) for (int i = 0; i < (n); ++i)

// 未AC

int main()
{
    int n, m, k;
    cin >> n >> m >> k;
    vector<vector<int>> g(n);
    rep(_, m)
    {
        int a, b;
        cin >> a >> b;
        a--, b--;
        g[a].push_back(b);
        g[b].push_back(a);
    }
    vector<int> h(n, -1);
    priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> q;
    rep(_, k)
    {
        int p;
        cin >> p;
        p--;
        cin >> h[p];
        q.push({h[p], p});
    }
    while (!q.empty())
    {
        auto p = q.top();
        q.pop();
        int u = p.second;
        if (h[u] > p.first)
            continue;
        if (h[u] <= 0)
            continue;
        for (int v : g[u])
        {
            if (h[v] >= h[u] - 1)
                continue;
            h[v] = h[u] - 1;
            if (h[v] > 0)
                q.push({h[v], v});
        }
    }
    vector<int> ans;
    rep(i, n) if (h[i] > -1) ans.push_back(i + 1);
    cout << ans.size() << endl;
    for (int x : ans)
        cout << x << endl;
}