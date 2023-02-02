# package.json
该文件用于定义项目所需要的各个模块及信息(名称，版本号，许可证等)。npm install命令会自动加载配置文件中的模块。也就是项目的运行和开发环境

##  交互式生产package.json文件
npm init
该命令会要求用户回答一些问题，根据回答生成文件，所有问题中只有name、version是必填的

##  基本字段
“name” : "项目名称"，
“version” : "1.0.0版本号",

##  scripts字段
该字段用于npm命令行缩写
“scripts”:{ 
    "login" : "node login.js",
    "msg" : "echo msg"
}
当执行npm run key 则执行对的value

##  dependencies，devDependencies字段
dependencies字段指定了项目*运行*所依赖的模块  
devDependencies指定项目*开发*所需要的模块。
“[dependencies，devDependencies]”:{    
    "browserify": "1.1.1",
    "karma-browserify": "2.2.2"
}
表示模块名，和版本要求；
当使用npm install的时候就会安装所需的模块

## main字段
该字段指定了加载入口的文件，require('moduleName')就会加载这个文件。默认值为根目录下的index.js

## bin字段
bin 用来对可执行文件添加内部命令
“bin”:{
    "binx" : "/xx.js"
}
npm binx命令就会执行xx.js, npm会在node_modules/.bin/目录下建立这个文件符号链接
node_modules/.bin/ （该目录会在运行时自动加入系统环境变量PATH）,所以运行npm命令时可以不带路径运行这些脚本，npm run [命令]

#config字段
该字段用于添加**命令行**的环境变量；使其他js文件可以访问该值
“config” : {
    "key" : "value"
}
npm config set 项目名:key value 用户可改变value

## style字段
"style" : ["/style.css"]
style指定供浏览器使用时，样式文件所在的位置。样式文件打包工具parcelify，通过它知道样式文件的打包位置。

browser  该字段指定该模板供浏览器使用的版本
engines  该字段指明了该模块运行的平台，比如 Node 的某个版本或者浏览器 或者npm的版本
preferGlobal  字段表示当用户不将该模块安装为全局模块时（即不用–global参数），要不要显示警告，表示该模块的本意就是安装为全局模块。



