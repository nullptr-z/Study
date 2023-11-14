#include "Sales.h"

Sales::Sales(istream& in)
{
	read(in, *this);
}

double Sales::avg__price() const
{
	if (unints_sold)
	{
		return revenue / unints_sold;
	}
	return 0.0;
}

Sales& Sales::combine(const Sales& sal)
{
	this->revenue += sal.revenue;
	this->unints_sold += sal.unints_sold;
	return *this;
}


Sales add(const Sales& sal1, const Sales& sal2)
{
	Sales sal = sal1;
	sal.combine(sal2);
	return Sales();
}

ostream& print(ostream& out, const Sales& sal)
{
	out << sal.isbn() << " " << sal.unints_sold << " " << sal.avg__price();
	return out;
}

istream& read(istream& in, Sales& sal)
{
	double price = 0;
	in >> sal.bookNo >> sal.unints_sold >> price;
	sal.revenue = price * sal.unints_sold;
	return in;
}
