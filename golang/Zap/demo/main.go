package main

import "go.uber.org/zap"

var logger *zap.Logger
var sulogger *zap.SugaredLogger

func main() {
	logger, _ = zap.NewDevelopment()
	sulogger = logger.Sugar()

	logger.Error("err", zap.String("x=", ""), zap.Error(err))
	logger.Info("err", zap.String("x=", ""))

	defer logger.Sync() // 退出时缓冲区的日志都刷到磁盘里
}
