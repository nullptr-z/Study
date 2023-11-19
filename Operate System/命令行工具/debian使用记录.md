终乱码端棱形
dpkg-reconfigure locales

给普通用户添加管理员权限
vi /etc/sudoers
-> zhou ALL=(ALL) ALL //给用户 zhou 添加管理权限

安装 deb 软件,解决依赖问题
dpkg -i _.deb //安装
apt-get -f install //如果安装中出现依赖问题,运行这一条命令,再次执行上面一条即可安装
mv /var/lib/dpkg/info/ /var/lib/dpkg/info_old/
mkdir /var/lib/dpkg/info/
apt-get update
apt-get -f install
mv /var/lib/dpkg/info/_ /var/lib/dpkg/info_old/
rm -rf /var/lib/dpkg/info
mv /var/lib/dpkg/info_old/ /var/lib/dpkg/info

安装 PDF
sudo apt-get install okular
sudo apt-get install kde-l10n-zhcn

开机自动挂载磁盘
cat /etc/fstab
UUID 或者完整路径 挂载到哪个目录 磁盘类型 noatime,nodiratime 0 2

第 5 列设置是否使用 dump 备份，置 0 为不备份，置 1，2 为备份，但 2 的备份重要性比 1 小
第 6 列设置是否开机的时候使用 fsck 检验所挂载的磁盘，置 0 为不检验，置 1，2 为检验，但置 2 盘比置 1 的盘晚检验。

关闭 xx-net 全局
install dconf-editor //安装
dconf-editor /system/http_proxy/ignore_hosts //关闭全局

让普通用户使用 root 权限命令
export PATH=$PATH:/sbin/
