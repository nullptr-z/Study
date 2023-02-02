##  使用nvm管理node版本
**安装nvm：**

curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.34.0/install.sh | bash

或者

wget -qO- https://raw.githubusercontent.com/creationix/nvm/v0.34.0/install.sh | bash

**nvm环境变量**
查看文件 ~/.bash_profile,.zshrc,.profile,.bashrc 
在文件尾部加上 exportNVM_DIR="$HOME/.nvm"[-s"$NVM_DIR/nvm.sh"]&&\."$NVM_DIR/nvm.sh"#This loads nvm

**安装node：**

nvm install stable


nvm 失效：~/.bashrct 添加 
[ -s "$NVM_DIR/nvm.sh" ] && . "$NVM_DIR/nvm.sh" # This loads nvm
    source ~/.bashrc更改立即生效

npm node 失效：nvm use --delete-prefix v11.10.1