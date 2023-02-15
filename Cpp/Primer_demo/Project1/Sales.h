#include <iostream>
using namespace std;
struct Sales {
private:
	//编号
	string bookNo;
	//销量
	unsigned unints_sold = 0;
	// 总收入
	double revenue = 0;
	// 平均价格
	double avg__price()const;

public:
	//构造函数
	Sales() = default;
	Sales(const string& str) :bookNo(str) {}
	Sales(const string& str, unsigned num, double price) :
		bookNo(str), unints_sold(num), revenue(price* num) { }
	Sales(istream&);

	// 成员函数
	string isbn() const { return bookNo; };
	Sales& combine(const Sales&);

	// 友员
	friend ostream& print(ostream&, const Sales&);
	friend istream& read(istream&, Sales&);
};

Sales add(const Sales&, const Sales&);

ostream& print(ostream& , const Sales& );

istream& read(istream&, Sales&);

