#include <numeric>
#include <iostream>
#include <vector>
using namespace std;

void AccumulateTestShow() {
	vector<string>   vct{ "def" ,"ghik" };
	auto substr = accumulate(vct.begin(), vct.end(), string("abc")); // |Á´½Ó×Ö·û´®
	cout << substr << endl;


	vector<int>   vct_Int{ 1,2,3,4,5 };
	auto sum = accumulate(vct_Int.begin(), vct_Int.end(), 1000); // ÇóºÍ
	cout << sum << endl;
}