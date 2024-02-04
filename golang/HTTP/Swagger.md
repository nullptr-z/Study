swager inti 生成文档

```go
import (
  // 一定要引入，否则启动后 doc.json not found
  _ "项目名/docs"
	swge "github.com/swaggo/files"
	swagger "github.com/swaggo/gin-swagger"
```

## 示例

```sh
// @Summary register User
// @Tags 用户
// @Param name formData string true "用户姓名"
// @Param pwd formData string true "密码"
// @Param repwd formData string true "确认密码"
// @Success 200 {string} json{code, success ,message}
// @Router /user/register [post]
```

```sh
// @Summary crete user
// @Schemes
// @Description crete user
// @Tags 用户
// @Accept json
// @Produce json
// @Param name query string true "用户名"
// @Param pwd query string false "密码"
// @Param email query string false "邮箱"
// @Success 200 {string} json{code,message}
// @Router /user/create [get]
```
