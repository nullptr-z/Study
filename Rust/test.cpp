#include <iostream>
#include <vector>
#include <queue>
#include <cmath>
using namespace std;

const int N = (1 << 20) + 10;
int n, m;
int v[N], w[N];
int res = -1;
int s[N]; // s[i]存储的是从第1件物品到第i件物品的价值总和

struct good
{
  int idx, c, r, tv; // idx表示选法下标，c表示该选法的当前总价值，r表示当前选法的剩余总价值，tv表示该选法的当前总体积
  bool operator>(const good &W) const
  {
    return W.c + W.r > c + r;
  }
} goods[N];

int bfs()
{
  goods[1] = {1, 0, 0, 0};

  priority_queue<good, vector<good>, greater<good>> q;
  q.push(goods[1]);

  while (q.size())
  {
    auto t = q.top();
    //        cout << t.idx << endl;
    q.pop();
    int idx = t.idx << 1;
    goods[idx] = {idx, goods[t.idx].c, s[n] - s[(int)log2(idx)], goods[t.idx].tv};
    goods[idx + 1] = {idx + 1, goods[t.idx].c + w[(int)log2(idx)], s[n] - s[(int)log2(idx)], goods[t.idx].tv + v[(int)log2(idx)]};
    if ((int)log2(t.idx) == n)
    { // 假如已经是子节点，则更新答案
      res = max(res, t.c);
      continue;
    }

    if (goods[idx].tv <= m && goods[idx].c + goods[idx].r > res)
      q.push(goods[idx]); // 假如当前选法的总体积不超过背包容量，且当前价值+剩余价值 > 当前最优解，则装入背包
    if (goods[idx + 1].tv <= m && goods[idx + 1].c + goods[idx + 1].r > res)
      q.push(goods[idx + 1]);
  }

  return res;
}

int main()
{
  cin >> n >> m;
  for (int i = 1; i <= n; i++)
    cin >> v[i] >> w[i], s[i] = s[i - 1] + w[i];
  //
  //    for(int i = 2; i < 1 << n + 1; i ++){
  //        goods[i] = {i,goods[i >> 1].c + (i&1)*w[(int)log2(i)],s[n] - s[(int)log2(i)],goods[i >> 1].tv + (i&1)*v[(int)log2(i)]};
  //    }
  //
  //    for(int i = 1; i < 1 << n + 1; i ++) printf("i = %d,c[i] = %d,r[i] = %d,tv[i] = %d\n",goods[i].idx,goods[i].c,goods[i].r,goods[i].tv);
  cout << bfs() << endl;
  return 0;
}
