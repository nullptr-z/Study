ym 软件仓库:
仓库定义在/etc/yum.repos.d 目录下,可以自行添加有效的 URL
yum repolist 查看已经加入的软件仓库(软件获取源)

查看已安装程序:
yum list installed 查看所有已安装程序
yum list 软件包 查看指定软件的详情
yum provides 文件名 查看指定文件属于哪个软件包;yum 会分别查找三个仓库：base、updates 和 installed。

安装软件:
yum install 软件包 从仓库中安装软件包,和他所有的依赖包
yum localinstall 软件包.rmp 从本地安装软件

更新软件:
yum list updates 查看所有需要更新的软件
yum update 更新所有软件
yum update 软件包 更新指定软件

卸载软件:
yum remove 软件包 只删除软件包而保留配置文件和数据文件
yum erase 软件包 删除软件和它所有的文件

处理损坏的包依赖关系:
有时在安装多个软件包时，某个包的软件依赖关系可能会被另一个包的安装覆盖掉。这叫作损坏的包依赖关系（broken dependency）。

清理更新软件包:
yum clean all

包的依赖关系:
yum deplist 软件名 这个命令显示了所有包的库依赖关系以及什么软件可以提供这些库依赖关系

yum update --skip-broken
--skip-broken 选项允许你忽略依赖关系损坏的那个包，继续去更新其他软件包。

============================================================

1 安装

yum install 全部安装

yum install package1 安装指定的安装包 package1

yum groupinsall group1 安装程序组 group1

2 更新和升级

yum update 全部更新

yum update package1 更新指定程序包 package1

yum check-update 检查可更新的程序

yum upgrade package1 升级指定程序包 package1

yum groupupdate group1 升级程序组 group1

3 软件包查找和详情

yum info package1 显示安装包信息 package1

yum list 显示所有已经安装和可以安装的程序包

yum list available 显示所有可用安装的程序包
　　　　　　 yum list installed 显示所有已安装的程序包

yum list package1 显示指定程序包安装情况 package1

yum groupinfo group1 显示程序组 group1 信息 yum search string 根据关键字 string 查找安装包

4 删除程序

yum remove package1 删除程序包 package1

yum groupremove group1 删除程序组 group1

yum deplist package1 查看程序 package1 依赖情况

5 清除缓存

yum clean packages 清除缓存目录下的软件包

yum clean headers 清除缓存目录下的 headers

yum clean oldheaders 清除缓存目录下旧的 headers

yum clean, yum clean all (= yum clean packages; yum clean oldheaders) 清除缓存目录下的软件包及旧的 headers
