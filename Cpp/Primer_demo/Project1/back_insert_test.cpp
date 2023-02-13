
#include <vector>
#include <algorithm>
#include <iostream>
using namespace std;

template<typename T>
void foreachVct(const vector<T>& vct) {
	for (auto& v : vct)
	{
		cout << v;
	}
	cout << endl;
}

template <typename T>
// ȥ���������ظ�
void quchongfu(vector<T> &vct, bool (*order)(T &, T &))
{
	sort(vct.begin(), vct.end(), order); // ˳������,�ֵ�����
	foreachVct(vct);

	// ȥ���ظ�,��Ч������ �ǰѲ��ظ�������ǰ������(�����ظ���),�������Ȳ���
	// �������һ�����ظ�λ�ú�һ��ָ��
	auto prt = unique(vct.begin(), vct.end());
	foreachVct(vct);

	// ɾ���ƶ���ΧԪ��,������ʼλ�����ظ��ʼ��λ��,�������ȼ����ظ���
	vct.erase(prt, vct.end());
	foreachVct(vct);
}

template<typename T>
bool dec(T& a, T& b) { return a > b; }

void back_insert_Show() {
	int arr1[] = { 7, 8, 9, 0, 3, 4, 1, 2, 2, 5, 6 };
	cout << sizeof arr1 << " " << sizeof(*arr1) << endl;
	int arr2[sizeof arr1 / sizeof(*arr1)];
	copy(begin(arr1), end(arr1), arr2); // ��arr1ȫ��������arr2


	vector<int> vct;
	copy(begin(arr2), end(arr2), back_inserter(vct));
	fill_n(back_inserter(vct), 5, 0);
	foreachVct(vct);
	quchongfu(vct, dec);
	cout << endl;

	vector<string> vctStr{ "cde-","cde-","pqrs-","fgh-","fgh-","ijkl-","ab-","mno-","xyz-","tuvw-" };
	foreachVct(vctStr);
	quchongfu(vctStr, dec);

	//���ݳ�������,�ұ���������ȵ��ֵ�����
	//stable_sort(vctStr.cbegin(), vct.cend()); // , [](string& a, string& b) {return a.size() > b.size(); }
}



