hostnamectl set-hostname

# 删除当前目录和子目录下× 所有.git文件夹 d代表目录 f代表文件
find * -type d -name ".git" -exec rm -rf {} \;

