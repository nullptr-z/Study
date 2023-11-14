#include <stdio.h>
#include <iostream>
#include <vector>
// #include "accumulate_test.cpp"
// #include "back_insert_test.cpp"
// #include "inserterStream_test.cpp"
// #include "lambda_test.cpp"
// #include "bind_ref_test.cpp"
// #include "new_malloc_test.cpp"
#include "vauleClass.h"
using namespace std;

std::string::size_type find_char(std::string& str, char ch, size_t& count);
// void AccumulateTestShow();
// void back_insert_Show();
// void showInserter();
// void lambda_test();
// void isMoreLen();
// void show_new_malloc_test();

int main() {
	// AccumulateTestShow();

	// back_insert_Show();

	// showInserter();

	// lambda_test();

	// isMoreLen();

	// show_new_malloc_test();
	vauleClass vcl("zhou", 24);
	cout << "姓名: "<< *vcl.str << "年龄: " << vcl.number << endl;
	vauleClass vcl1(vcl);
	cout << "姓名: "<< *vcl.str << "年龄: " << vcl.number << endl;
	vcl = vcl1;
	
	return 0;
}
