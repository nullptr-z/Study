#include <iostream>
using namespace std;

class vauleClass
{
private:
  /* data */

public:
  string *str;
  int number;

  vauleClass(const string &s, const int &num)
      : str(new string(s)), number(num){};

  vauleClass(const vauleClass &vcl)
      : str(new string(*vcl.str)), number(vcl.number)
  {
    cout << "调用了拷贝构造函数" << endl;
  };

  vauleClass &operator=(const vauleClass &vcl)
  {
    cout << "调用了拷贝赋值运算符" << endl;
    auto newStr = new string(*vcl.str);
    delete str;
    this->str = newStr;
    this->number = vcl.number;
    return *this;
  }

  ~vauleClass()
  {
    delete str;
    cout << "调用了析构函数" << endl;
  };
};
