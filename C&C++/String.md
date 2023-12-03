现代 C++应该尽量使用 vector 和迭代器，避免使用内置的数组和指针——指针代码阅读性差烦琐，容易出现不易发现的错误。指针多用于底层；尽量用 string，避免使用 C 风格基于数组的字符串
容器的目的:为程序员提供元素的存储和访问能力

String:
初始化：
默认初始值为空字符串
string str(3,'c') 初始值为 ccc ;
string str("cc")

str.empty() 为空 true, 否则 false
str.size() 字符个数——返回 size_type 无符号类型 不要将其和带符号类型作比较（可能会被转化成一个很大的无符号数）
str[n] n 为下标
str1 + str2 连接字符串 ，其中一个也可以是字面值
getline(str1, str2) 从 str1 从读取一行给 str2 遇到换行符为止
== != < > <= >= 按字符的 ascii 比较很容易得出答案 ;如:b>a = 97>96

zhou zheng
cin>>str1 >>str2 从第一个非空白字符开始读取 str1=zhou, str2=zheng

[] 下标(属于引用访问),如果访问不存在的下标,错误：未定义的

vector 模板
vector<string> s（10）; 10 个默认初始化的元素
vector<string> s（10，"t"）; 10 个值为 t 的元素
vector<int> s{10}; 1 个元素初始值为 10
通过[]可以访问已存在元素，而不能下标添加元素
s.push_back(str); 在尾端添加 str
