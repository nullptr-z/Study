实际上不同数据库之间的迁移代价非常大，虽然 ORM 可以消除一些障碍，但是不如已开始就选择好数据库

## 作用

将数据库表结构映射到编程语言的结构对象

帮助开发者快速的操作数据库，忽略数据库一些复杂的细节

## 优点

- 提高开发效率，和可维护性
- 更换底层数据库，迁移成本很低
- 屏蔽 sql 细节，不必学习复杂 sql 语句

## 缺点：

- 通常 ORM 库都比较重，庞大，效率受损
- 可能过多屏蔽了数据库一些好的功能
- ORM 用久了，sql 会被遗忘；如果换一个 ORM 框架，甚至换到另一种语言，要学习新的 ORM Api
