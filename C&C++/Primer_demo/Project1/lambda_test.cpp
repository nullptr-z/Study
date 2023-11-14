#include <vector>
#include <iostream>
using namespace std;

template<typename Lamd>
void showNumber(Lamd lamd){
    lamd(5);
}

void lambda_test(){
    unsigned num = 11;

    showNumber([&num](unsigned inc) -> void{ // 这里是【引用捕获还是传地址】还没搞清楚
        cout << num << endl;
        num += inc;
    });
    cout << num << endl;
}