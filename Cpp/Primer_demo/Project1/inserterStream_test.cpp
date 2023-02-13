#include <vector>
#include <iostream>
#include <algorithm>
#include <iterator>
#include <new>
using namespace std;

template<typename T>
void foreachVct(const T& vct) {
	for (auto v = vct.cbegin(); v != vct.cend(); ++v)
	{
		cout << *v;
	}
	cout << endl;
}

template<typename T>
void rforeachVct(const T& vct) {
	for (auto v = vct.crbegin(); v != vct.crend(); ++v)
	{
		cout << *v;
	}
	cout << endl;
}

void shared(shared_ptr<int> sp) {
	cout << sp.unique() << endl;
}

void showInserter() {
	cout << endl << "--------inserterStream_test-------" << endl;
	vector<unsigned> vInt{ 1,2,3,4,5 };
	vInt.insert(++vInt.begin(), 9);
	foreachVct(vInt);

	auto inserterIn = inserter(vInt, vInt.begin());
	*inserterIn = 8;
	auto  back = back_inserter(vInt);
	*back = 6;

	string str("ab:cd:efg");
	foreachVct(str);
	auto findResultIn = find(str.crbegin(), str.crend(), ':');
	foreachVct(string(str.crbegin(), findResultIn)); // 逆序的
	rforeachVct(string(str.crbegin(), findResultIn)); // 正序一
	foreachVct(string(findResultIn.base(), str.cend())); // 正序二

	auto p = new int(5);
	shared_ptr<int> ps1(p);
	auto ps2(ps1);
	cout << *ps2<< endl;
	cout << ps2.unique() << endl; // 计时器为2
	ps1.reset(new int(5));
	cout << ps2.unique() << endl;// 计时器为1
	//shared(shared_ptr<int>(p)); // ps2悬空指针
	//cout << *ps2 << endl;
}