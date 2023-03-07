关闭默认启动图形界面
sudo systemctl disable lightdm

查看桌面环境状态 �
��sudo service lightdm status

启动桌面环境 �
��sudo service lightdm start

停止桌面环境
：sudo service lightdm stop

重启桌面环境 �
��sudo service lightdm restart

systemctl daemon-reload // 重新加载 systemctl service 配置

开机进入命令模式

1. LEVLE 3

systemctl set-default multi-user.target
Copy the Code

2. LEVEL 5

systemctl set-default graphical.target

误删 python2, 桌面黑屏，文件管理器 商店没有了等问题
https://blog.csdn.net/hulang_better_me/article/details/99620350
