关闭默认启动图形界面
sudo systemctl disable lightdm

查看桌面环境状态�
��sudo service lightdm status

启动桌面环境�
��sudo service lightdm start

停止桌面环境
：sudo service lightdm stop

重启桌面环境�
��sudo service lightdm restart


systemctl daemon-reload // 重新加载 systemctl service配置



开机进入命令模式
1. LEVLE 3

systemctl set-default multi-user.target
Copy the Code

2. LEVEL 5

systemctl set-default graphical.target
