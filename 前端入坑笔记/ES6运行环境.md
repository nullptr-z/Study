## gulp服务器
**安装 gulpx**
*安装在本地会找不到gulp指令*
npm i -D gulp
npm i -D gulp-connect

**新建 gulpfile.js 文件**
---
var gulp = require('gulp'),
    connect = require('gulp-connect');
gulp.task('webserver', function() {
    connect.server({
        livereload: true,
        port: 8088
    });
});
gulp.task('default', ['webserver']);
---

npm init -yes  生成package.json

并添加依赖包：
"devDependencies": {
    "babel-polyfill": "^6.9.1",
    "babel-preset-es2015": "^6.6.0",
    "gulp": "^3.9.1",
    "gulp-babel": "^6.1.2",
    "gulp-browserify": "^0.5.1",
    "gulp-connect": "^3.2.3",
    "gulp-rename": "^1.2.2",
    "gulp-sync": "^0.1.4",
    "gulp-uglify": "^1.5.3"
  }
