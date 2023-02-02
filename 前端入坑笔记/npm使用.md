*允许用户从NPM服务器下载别人编写的第三方包到本地使用。*
*允许用户从NPM服务器下载并安装别人编写的命令行程序到本地使用。*
*允许用户将自己编写的包或命令行程序**上传**到NPM服务器供别人使用*

---
npm install 该命令会安装package.json文件中表明所需要的模块

npm install express [--save, --save-dev, -g]

命令表示安装express,并将--save参数写入dependencies;
--save-dev参数写入devDependencies
-g参数带表明是全局安装

安装好的 express 直接被放在工程目录下的node_modules中（拥有系统环境变量），直接require('express')使用无须其他路径
var express = require('express');

---
npm uninstall express 卸载模块

---
npm update express 更新模块

---
npm ls 查看所有模块

---
npm search express 查看指定模块

## 模块获取源
除了官方默认的还有第三方的
npm install -g cnpm --registry=https://registry.npm.taobao.org 这是淘宝源