本地Git仓库和GitLab仓库之间的传输是通过SSH加密的，所以必须要让github仓库认证你SSH key，在此之前，必须要生成SSH key

sudo /etc/init.d/ssh start   启动ssh

#  生成sshKey
    ssh-keygen -t rsa -C "密钥注释字段"
ssh-keygen命令用于为“ssh”生成、管理和转换认证密钥，它支持RSA和DSA两种认证密钥。

ssh-keygen(选项)
选项
-b：指定密钥长度； -e：读取openssh的私钥或者公钥文件；
-C(大写)：添加注释； -f：指定用来保存密钥的文件名；
-l：显示公钥文件的指纹数据； -i：读取未加密的ssh-v2兼容的私钥/公钥文件，然后在标准输出设备上显示openssh兼容的私钥/公钥；
-N：提供一个新密语； -P：提供（旧）密语；
-q：静默模式； -t：指定要创建的密钥类型。

1、密钥类型 -t 选项指定。如果没有指定则默认生成用于SSH-2的RSA密钥。这里使用的是rsa。

2、密钥中有一个注释字段，用-C来指定所指定的注释，可以方便用户标识这个密钥，指出密钥的用途或其他有用的信息，所以在这里输入自己的邮箱或者其他都行。

输入完毕后程序同时要求输入一个密语字符串(passphrase)，空表示没有密语。接着会让输入2次密码(password)，空表示没有密码。3次回车即可完成当前步骤，此时根>用户名>.ssh目录下已经生成好了。

将命令复制到gitlab账号上
登录gitlab。点击头像打开settings->SSH keys把生成的公钥id_rsa.pub里面的内容放进key输入框中，再为当前的key起一个title来区分每个key点击Add Key即可

# 下载仓库代码到本地

   git clone 仓库地址

# 分支操作
git checkout -b zz //创建名为dev的分支，并且切换到dev分支

git checkout - //切换到上一分支 

git branch //查看所有分支，高亮显示当前分支

git branch zz //创建名为dev的分支

git log --graph //分支详情

# 拉取仓库代码
git pull

git merge master //将代码同步到当前分支

#  文件更改
git status //查看改动项

git add fileName //添加文件信息

git commit -m "msg"  //msg为本次提交的概述信息

git log //查看提交日志

git push  //同步到仓库

git rm //删除
# 回溯操作
git reset //回溯历史的版本

git relog //查看仓库的操作日志，推进历史的哈希值

git reset --hrad  has//回溯到指定状态，has为历史版本的哈希值



# 管理自己的仓库
## 创建
创建秘钥时勾选允许推送到此仓库
为不同的仓库添加不同的秘钥,最好创建秘钥时取个很好辨识的名字

clone提示权限不足时,
ssh -T git@github.com 测试公钥是否添加成功时，不成功提示：git@github.com: Permission denied (publickey)
解决办法1:重新生成秘钥,别取名字

## 删除
git branch -d 分支名 删除本地分支
git push origin --delete 分支名 删除远程分支
