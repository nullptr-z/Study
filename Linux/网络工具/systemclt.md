搭建 frphttps://www.jianshu.com/p/a621556fc07b使用systemctl来控制启动

这个方法比较好用，很方便

sudo vim /lib/systemd/system/frps.service

在 frps.service 里写入以下内容

```sh
[Unit]
Description=fraps service
After=network.target syslog.target
Wants=network.target

[Service]
Type=simple #启动服务的命令（此处写你的 frps 的实际安装目录）
ExecStart=/your/path/frps -c /your/path/frps.ini

[Install]
WantedBy=multi-user.target
```

然后就启动 frps

sudo systemctl start frps

再打开自启动

sudo systemctl enable frps
如果要重启应用，可以这样，sudo systemctl restart frps
如果要停止应用，可以输入，sudo systemctl stop frps
如果要查看应用的日志，可以输入，sudo systemctl status frps
