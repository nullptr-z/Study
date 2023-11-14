#include <new>
#include <iostream>
using namespace std;

shared_ptr<int> shared_test()
{
  auto *pr = new int(3);
  // delete pr;
  // shared_ptr<int> prS(pr);
  // cout<<prS.unique()<< endl;

  return shared_ptr<int>(pr);
}

void show_new_malloc_test(){
  auto s = shared_test();
  cout << *s << "   s.unique()" << s.unique() << endl;
  cout << endl;
}