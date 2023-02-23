# launchctl

# 加载任务, -w选项会将plist文件中无效的key覆盖掉，建议加上
$ launchctl load -w com.demo.plist

# 删除任务
$ launchctl unload -w com.demo.plist

# 查看任务列表, 使用 grep '任务部分名字' 过滤
$ launchctl list | grep 'com.demo'

# 开始任务
$ launchctl start  com.demo.plist

# 结束任务
$ launchctl stop   com.demo.plist

plist部分参数说明：
Label：对应的需要保证全局唯一性；
Program：要运行的程序；
ProgramArguments：命令语句
StartCalendarInterval：运行的时间，单个时间点使用dict，多个时间点使用 array <dict>
StartInterval：时间间隔，与StartCalendarInterval使用其一，单位为秒
StandardInPath、StandardOutPath、StandardErrorPath：标准的输入输出错误文件，这里建议不要使用 .log 作为后缀，会打不开里面的信息。
定时启动任务时，如果涉及到网络，但是电脑处于睡眠状态，是执行不了的，这个时候，可以定时的启动屏幕就好了。

链接 https://www.jianshu.com/p/4addd9b455f2



# systemctl

systemctl daemon-reload  // 修改*.service后重载systemctl
