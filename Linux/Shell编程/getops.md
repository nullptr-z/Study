getopt -p ab:cd -a -b parm -c -d -- test1 test2 test3
getopts "ab:cd” opt //这里的 opt 表示实例对象
/// 在一个选项后面加冒号:表示有一个参数; 在最前面加冒号表示不抛出错误
OPTARG 保存着选项的当前选型的参数; --似乎是对每一个选项都尝试读取参数,如果没有参数 OPTARG 就是空的,好像一个选项最多一个参数
OPTIND 保存了选项的总个数

选项标准化:
正统的和一些良好的脚本约定使用以下标识选项:
-a 显示所有对象 --all
-c 生成一个计数 --count
-d 指定一个目录 --dir
-e 扩展一个对象 --expand
-f 指定读入数据的文件 --file
-h 显示命令的帮助信息 --help
-i 忽略文本大小写 --ignore
-l 产生输出的长格式版本(详情) --long
-n 使用非交互模式(批处理) --
-o 将所有输出重定向到的指定的输出文件 --out file
-q 以安静模式运行 --quiet
-r 递归地处理目录和文件 --recursive
-s 以安静模式运行 --
-v 生成详细输出 --view
-x 排除某个对象 --
-y 对所有问题回答 yes --默认同意

echo -n 输出时不换行
读取输入:
read ...变量 该命令可以从读取用户的输入,并且保存在变量中, 以空格分隔可以将输入赋值给多个变量,多余的参数被赋值最后一个变量
-p msg 输出 msg 表示提示信息
-t 设置一个等待输入时间
-s 隐藏输入,例如获取密码输入时可以使用

可以使用管道将文件内容赋值给 read 命令, 读取到换行符则停止
