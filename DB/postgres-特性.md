## Range

`交、并、差`集

运算： 部分重叠，完全重叠，重叠部分有哪些

## CONSTRAINT

新加入的数据必须满足某个条件，否则不能插入

## EXCLUDE

`排他约束`，用于保证某个范围内的值不重复

新插入的行不能与已有的所有行有重叠，类似于唯一约束，但是唯一约束只能保证某个字段的值不重复，而排他约束可以保证某个范围内的值不重复

## gist-索引

`复合类型的索引结构`，使用 RANGE 类型

## 例子

```sql
# 通过索引结构 gist 来保证新插入的行的 lower_bound 和 upper_bound 不能与已有的行有重叠
CONSTRAINT test_exclusion_constraint EXCLUDE USING gist (
    int4range(lower_bound, upper_bound, '[]') WITH &&,
    int4range(lower_bound, upper_bound, '[]') WITH &&
)
```

## Index-索引

123
