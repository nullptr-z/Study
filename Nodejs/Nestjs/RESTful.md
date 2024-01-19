## 版本控制

全局添加版本控制

http://localhost:3005/v1/demo1/3 类似这样的效果 v1 是自动添加的

```ts
import { VersioningType } from "@nestjs/common";

const app = await NestFactory.create(AppModule);
app.enableVersioning({
  type: VersioningType.URI,
});

// controller
@Controller({
  path: 'demo1',
  version: '1'
})

// type决定将版本号放在什么地方
export declare enum VersioningType {
  URI = 0, // 在路由里添加；最常用
  HEADER = 1, // 请求头
  MEDIA_TYPE = 2,
  CUSTOM = 3, // 自定义
}
```

## 函数装饰器单独做版本控制

```ts
// ..Controller.ts
@Version('2')
function getId(){...}
```

不能用于 Controller 控制器
