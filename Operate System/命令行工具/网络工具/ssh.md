上传文件到服务器
scp fileName servername@ip:serverDir

从服务器下载文件
scp servername@ip:serverDir localDir

//scp 命令常用选项
-r //递归复制
-a //全部复制
-p //保持权限
-P //端口
-q //静默模式

创建公私秘钥
ssh-keygen -t rsa

注意事项:
一个 ssh 程序分为 client 端和 server 端
大多数发行版 linux 默认都有 client 端,server 端则不一定有,centos 广泛作为服务器系统,通常两个都有,所以在使用时检查当前所用的发行版是否安装了需要的部分
sudo apt install openssh-server #安装 server 端 ssh

有时还需要注意防火墙

# 以下 default XX 代表没写或被注释了该属性属于什么状态

服务端/etc/ssh/sshd_config
Port 22 #设置 ssh 端口,默认 22

PermitRootLogin --default no #是否允许登录 root;当状态为 no,配置秘钥或者拥有密码的用户都无法登入
PasswordAuthentication --default yes #是否启用密码登录认证 --状态为 no 时客户端得到信息:Permission denied(publickey)
当输入密码后客户端会在.ssh/目录下 know_hosts 文件存放服务器信息
RSAAuthentication #是否启用 rsa 秘钥认证,SSH 的通讯协议第一代,功能和安全不如第二代,不太明白机制
PubkeyAuthentication --default yes #是否启用秘钥认证(第二代);
注解:公钥比喻为一把锁,私钥为钥匙,用户可以把公钥保存在服务器,每次访问时使用私钥来开锁
服务器存储公钥的方法:
方法一:通常使用 ssh-copy-id serverName@IP -p 端口号(如果不是默认 22 则要手动填写),向服务器发送公钥
方法二:用户登入服务器后手动创建.ssh/authorized_keys 文件吧公钥复制进去
方法三:通过外设拷贝秘钥,其他

AuthorizedKeysFile #设置存放用户公钥的文件位置;这个文件权限通常应设为 600

重启 ssh 服务:
如果配置文件有错误时，用户会无法连接，重启服务时能知道错误
service sshd restart

客户端/etc/ssh/ssh_config

Host [name] ＃　直接使用 ssh name 登入服务器
HostName 140.206.185.170 ＃　服务器ＩＰ
Port 2223 ＃　服务器端口
User root ＃　登入用户
IdentityFile ~/.ssh/id_rsa ＃　指定使用哪个秘钥开锁
PreferredAuthentications publickey ＃　使用什么方式登录，publickey 秘钥登录，password 密码登录

防火墙:
sudo chkconfig iptables off 永久关闭
sudo service iptables stop 将临时关闭
