#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (n); ++i)

int main()
{
    int h, w;
    cin >> h >> w;
    vector<string> s(h);
    rep(i, h) cin >> s[i];

    int x = 0, y = 0;
    rep(i, h) rep(j, w)
    {
        if (s[i][j] == '#')
        {
            continue;
        }
        int cnt = 0;
        if (i > 0 && s[i - 1][j] == '#')
            cnt++;
        if (i < h - 1 && s[i + 1][j] == '#')
            cnt++;
        if (j > 0 && s[i][j - 1] == '#')
            cnt++;
        if (j < w - 1 && s[i][j + 1] == '#')
            cnt++;
        if (cnt >= 2)
        {
            x = i + 1;
            y = j + 1;
        }
    }
    cout << x << " " << y << endl;
}