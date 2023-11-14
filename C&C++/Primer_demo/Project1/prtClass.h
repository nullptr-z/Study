using namespace std;
class prtClass
{
private:
  string *str;
  size_t *use;

public:
  prtClass(const string &str)
      : str(new string(str)), use(new size_t(1)){};

  prtClass(prtClass &pcl)
      : str(pcl.str), number(pcl.use)
  {
    cout << "调用了拷贝构造函数" << endl;
    ++*use;
  };

  prtClass &prtClass(prtClass &pcl)
  {
    cout << "调用了移动赋值运算符" << endl;
    ++*pcl.use;
    if (--*use == 0)
    {
      delete use;
      delete str;
    }
    str = pcl.str;
    use = pcl.use;
    return *this;
  }

  void swap(prtClass &fPcl, prtClass &lPcl)
  {
  }

  ~prtClass(){
    if (--*use == 0)
    {
      delete use;
      delete str;
    }
  };
};

prtClass::prtClass(/* args */)
{
}

prtClass::~prtClass()
{
}
