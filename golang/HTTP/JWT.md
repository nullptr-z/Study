## JSON Web Token

一种 json 格式认证协议，token 串就是 json 的，不过生产时候编码成了 base64，解码极其简单

不同于 Cookie 和 Session，jwt token 不占用任何客户端和服务器存储空间

信息都包含在 Token 串里面，所以不要包含密码等敏感信息，一般包含内容能识别用户即可，

三部分组成:

- Header: 包含了加密类型.
- Payload: 签发/过期时间、自定义内容..
- Signature：包含秘钥，使用秘钥对前两部分加密，如果前面两部分内容产生了更改，加密出来的内容是对应不上的，_不可伪造_

```go
// payload
type StandardClaims struct {
	Audience  string `json:"aud,omitempty"`
	ExpiresAt int64  `json:"exp,omitempty"`
	Id        string `json:"jti,omitempty"`
	IssuedAt  int64  `json:"iat,omitempty"`
	Issuer    string `json:"iss,omitempty"`
	NotBefore int64  `json:"nbf,omitempty"`
	Subject   string `json:"sub,omitempty"`
}
```

## Access & Refresh Token

Access Token 有效期一般不会设置很长，因为被盗用了就很麻烦，服务端无法让他过期，只能添加 Token 黑名单，这有增加了服务器开销
Refresh Token 有效期长，不直接负责验证处理，Access 过期而 Refresh 有效时，会重新生成 Access 下发给客户端

## 安全性

尽管 JWT 本身为信息传递提供了一种安全方式，但仍需注意安全最佳实践：

- 使用 HTTPS 来防止中间人攻击。
- 保证 JWT 的签名安全，不要使用弱密钥，对于敏感应用应使用 RSA 等公钥/私钥对加密方式。
- JWT 不应包含敏感数据，因为 payload 是可以被解码的，除非它使用不可逆加密算法。
- 有意识地处理 JWT 的过期时间，以减少令牌被滥用的风险。
- JWT 提供了一种有效的方式来确保数据的完整性和安全性，但它的安全性也依赖于实施和使用时的注意事项。

## 生成秘钥

[生成 key](https://1password.com/zh-cn/password-generator/)
