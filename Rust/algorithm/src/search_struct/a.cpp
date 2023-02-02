#include <iostream>
#include <vector>
#include <fstream>
#include <queue>

using namespace std;

/*数据格式
价值：40,42,25,12
重量：4, 7, 5, 3
*/

class Solution
{
private:
  //私有类
  //该结点为扩展结点信息
  class Object
  {
    friend Solution;

  private:
    int id;       //用于区分哪一个物品
    float value;  //价值
    float weight; //质量
    float ratio;  //单位重量价值
    void calcRatio()
    {
      ratio = value / weight;
    }
  };

  //活结点树结点
  class subnode
  {
  public:
    friend Solution;

  private:
    subnode *parent; //指向父结点
    bool LChild;     //是否是左儿子，即是则代表放入该节点
  };

  //优先级队列结点
  class HeapNode
  {
    friend Solution;

  public:
    //重载转换符,返回上界作为堆比较基础
    operator float() const
    {
      return upperBound;
    }

  private:
    float upperBound;    //结点处的价值上界
    float currentValue;  //结点处的当前价值
    float currentWeight; //记录当前重量
    int level;           //活结点在活结点数上的层数
    subnode *ptr;        //指向活结点树上的应结点的指针
  };

  //存储基本信息
  priority_queue<HeapNode> priorityHeap; //优先级队列，即活结点表
  subnode *liveTree;                     //活结点树
  float capacity;                        //能装载的总质量
  int num;                               //物品总数
  vector<Object> items;                  //物品集合
  float currentWeight;                   //当前重量
  float currentValue;                    //当前价值
  bool *bestSolution;                    //最优策略
  //类函数，用于比较方法
  struct compare
  {
    //用于sort中自定义比较方法,从单位质量的价值从大到小排序
    bool operator()(Object pre, Object post) const
    {
      return pre.ratio > post.ratio;
    }
  };
  //方法
  //结点加入活结点树与优先级队列中（堆）
  void addLiveNode(float upperBound, float currentValue, float currentWeight, bool ch, int level);
  //计算当前物品的上界
  float calcupperBound(int i);

public:
  //构造函数，输入数据，与物品数目，背包容量
  Solution(vector<float> &data, int num, float capacity);
  //优先级队列分枝限界法，返回最大价值与最优策略
  float solveSack();
  //输出结果
  void result();
};

Solution::Solution(vector<float> &data, int num, float capacity)
{
  //赋值
  this->num = num;
  this->capacity = capacity;
  items = vector<Object>(num);
  bestSolution = new bool[num];
  for (int i = 0; i < num; i++)
  {
    items[i].id = i;
    items[i].value = data[i];
    items[i].weight = data[i + num];
    items[i].calcRatio();
  }
  sort(items.begin(), items.end(), compare());
  //初始化数据
  liveTree = NULL;
  currentWeight = 0;
  currentValue = 0;
}

//结点加入到活结点树与优先级队列中（堆）
void Solution::addLiveNode(float upperBound, float currentValue, float currentWeight, bool Lchild, int level)
{
  subnode *node = new subnode;
  //结点放入活结点树
  node->parent = liveTree;
  node->LChild = Lchild;
  //准备即将放入优先级队列的结点,同步当前信息
  HeapNode heapNode;
  heapNode.upperBound = upperBound;
  heapNode.currentValue = currentValue;
  heapNode.currentWeight = currentWeight;
  heapNode.level = level;
  heapNode.ptr = node;
  //加入优先级队列中
  priorityHeap.push(heapNode);
}

//第i个物品的价值以及其所对应值的上界
float Solution::calcupperBound(int i)
{
  //剩余容量
  float restCapacity = capacity - currentWeight;
  //价值上界
  float upperBound = currentValue;
  //将剩余重量可放入的物品放入
  //物品须按照从大到小排列
  while (i < num && items[i].weight <= restCapacity)
  {
    restCapacity = restCapacity - items[i].weight;
    upperBound = upperBound + items[i].value;
    i++;
  }
  //当前物品装不下时，且不为叶节点时，将物品切割为至等于剩余重量，然后计算出当前的最大上界（由于是0-1背包，实际上无法切割）
  if (i < num)
  {
    upperBound = upperBound + items[i].ratio * restCapacity;
  }
  return upperBound;
}

//优先级队列分枝限界法，返回最大价值与最优策略
float Solution::solveSack()
{
  int i = 0;           //设置当前寻找结点
  float localBest = 0; //当前局部最优值
  //获取当前价值上界
  float upperBound = calcupperBound(i);
  while (i < num)
  { //不是叶节点
    //加入当前结点
    //检查当前扩展结点的左儿子结点
    float weight = currentWeight + items[i].weight;
    if (weight <= capacity)
    { //左儿子结点可行
      //大于局部的最优值
      if (currentValue + items[i].value > localBest)
      {
        localBest = currentValue + items[i].value;
      }
      //加入结点
      addLiveNode(upperBound, currentValue + items[i].value, currentWeight + items[i].weight, true, i + 1);
    }
    //不加入当前结点
    upperBound = calcupperBound(i + 1);
    //检查当前扩展结点的右儿子结点
    if (upperBound >= localBest)
    { //若大于，此时可能含有最优解
      addLiveNode(upperBound, currentValue, currentWeight, false, i + 1);
    }
    //取出队首结点，
    HeapNode heapNode;
    //将当i前结点heapNode从堆中取出
    heapNode = priorityHeap.top();
    //删除
    priorityHeap.pop();
    //获取结点树
    liveTree = heapNode.ptr;
    //获取当前结点处的重量
    currentWeight = heapNode.currentWeight;
    //获得当前结点处的价值
    currentValue = heapNode.currentValue;
    //获取当前结点处的上界
    upperBound = heapNode.upperBound;
    //获取层次
    i = heapNode.level;
  }
  //获取当前最优解
  for (int j = num - 1; j >= 0; j--)
  {
    bestSolution[j] = liveTree->LChild;
    liveTree = liveTree->parent;
  }
  return currentValue;
}

void Solution::result()
{
  cout << "最大价值为:" << currentValue << endl
       << "策略为:" << endl;
  for (int i = 0; i < num; i++)
  {
    if (bestSolution[i])
    {
      cout << (char)('A' + items[i].id) << " ";
    }
  }
}

int main()
{
  vector<float> data;  //记录数据
  int num = 0;         //记录节点数目
  float capactity = 0; //记录背包容量
  ifstream ifs("data.txt", ios_base::binary);
  if (!ifs)
  {
    return -1;
  }
  while (!ifs.eof())
  {
    float dig;
    ifs >> dig;
    if (ifs.fail())
    {
      ifs.clear();
      ifs.ignore(1);
      continue;
    }
    num++;
    data.push_back(dig);
  }
  ifs.close();
  ifs.clear();
  num = num / 2;
  cout << "请输入背包容量:" << endl;
  cin >> capactity;
  cout << endl;
  Solution solve(data, num, capactity);
  data.clear();
  solve.solveSack();
  solve.result();
}
