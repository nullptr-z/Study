# 第三方 golang 运行环境的镜像
FROM golang:alpine
# 定义环境变量
ENV GO111MODULE = on \
  GOPROXY = https://goproxy.cn \
  GOOS = linux \
  GOARCH = amd64
# 设置工作目录为 /build
WORKDIR /build
# 将当前目录代码内容复制到容器中 /app
COPY . .
# 运行本地脚本
RUN go build -o app .
# 切换到目录,并把可执行文件复制过来
# WORKDIR /dist
# RUN cp /build/app .
# 安装 requirements.txt 中指定的任何所需包
# RUN go get package
# 使端口 8888 可供此容器外的环境使用; 也就是服务端口
EXPOSE 8899
# 在容器启动时运行 app
CMD ["/build/app"]
