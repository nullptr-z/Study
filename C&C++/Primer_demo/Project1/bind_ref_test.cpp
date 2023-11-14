#include <iostream>
#include <algorithm>
#include <string>
#include <functional>
#include <vector>
using namespace std;
using namespace std::placeholders;

bool check_size(const string& str,size_t sz,unsigned& count) {
	++count;
	cout << "第" << count << "次调用" << endl;
	return str.size() > sz;
}

template<typename T>
class refs {
public:
	refs(T& t) :tt(t) {};

	T& tt;

	refs& operator++() {
		this->tt++;
		return *this;
	}
};

void isMoreLen() {
	cout << endl << "--------bind_ref_test-------" << endl;
	const vector<string> vstr{
		string("ab"),
		string("abc"),
	};
	size_t sz = 2;
	unsigned count = 0;
	auto rr = refs<unsigned>(count);
	auto result = find_if(vstr.begin(), vstr.end(), bind(check_size, _1, sz, ref(count)));
	cout << "大于sz的为:" << *result << "调用了check_size:" << count << "次" << endl;
	size_t i = 0;
	for (auto b = vstr.begin(); b != vstr.end(); ++b, ++i)
	{
		if (b==result)
		{
			cout << "下标为:" << i << endl;
		}
	}

	++rr;
	cout << "refs++结果" << count << endl;

	//auto mutablesd = [count]() {return count += 2; }; // 错误
	auto mutablesd = [count]()mutable {return count += 2; };
	mutablesd();
	cout << "mutable结果" << count << endl;
	auto yiyong = [&count]() {return count += 2; };
	yiyong();
	cout << "引用结果" << count << endl;
}