#include <iostream>
using namespace std;
struct Sales {
private:
	//���
	string bookNo;
	//����
	unsigned unints_sold = 0;
	// ������
	double revenue = 0;
	// ƽ���۸�
	double avg__price()const;

public:
	//���캯��
	Sales() = default;
	Sales(const string& str) :bookNo(str) {}
	Sales(const string& str, unsigned num, double price) :
		bookNo(str), unints_sold(num), revenue(price* num) { }
	Sales(istream&);

	// ��Ա����
	string isbn() const { return bookNo; };
	Sales& combine(const Sales&);

	// ��Ա
	friend ostream& print(ostream&, const Sales&);
	friend istream& read(istream&, Sales&);
};

Sales add(const Sales&, const Sales&);

ostream& print(ostream& , const Sales& );

istream& read(istream&, Sales&);

