hostnamectl set-hostname

# 删除当前目录和子目录下× 所有.git文件夹 d代表目录 f代表文件
find * -type d -name ".git" -exec rm -rf {} \;

# 统计文件格式
ls -l resource/v8/ | grep ^- | wc -l

## 输出目录结构树
find . -type d -name "ignore" -prune -o -print | tree -F --fromfile
